use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use aws_sdk_ec2::types::{
    BlockDeviceMapping, EbsBlockDevice, InstanceType, LaunchTemplateSpecification, ResourceType,
    Tag, TagSpecification, VolumeType,
};
use aws_sdk_ec2::{Client, Error};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;
use std::str::FromStr;

pub(crate) async fn run_instance(client: Client, webhook: Webhook) -> Result<String, Error> {
    let repository_full_name = &webhook.repository.full_name;
    let workflow_job_id = webhook.workflow_job.id.to_string();
    let workflow_run_id = webhook.workflow_job.run_id.to_string();

    // TODO: Get cloud-init to do this
    let user_data = BASE64_STANDARD.encode(format!("#!/bin/sh

add-apt-repository ppa:ansible/ansible # https://github.com/ansible/ansible/issues/77624
apt-get update
apt-get -y install ansible-core
apt-get clean
ansible-galaxy collection install amazon.aws community.general
ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --extra-vars 'url=https://github.com/{}' --extra-vars 'token={}' ansible/runner.yml"
    , &repository_full_name, create_registration_token_for_repository(&repository_full_name, &webhook)));

    let mut instance_type = InstanceType::M7gLarge;
    let mut volume_size: i32 = 30; // This can fit in an u16

    for label in webhook.workflow_job.labels {
        if label.starts_with("EC2-") {
            instance_type = InstanceType::from_str(label.strip_prefix("EC2-").unwrap()).unwrap();
        } else if label.starts_with("EBS-") {
            volume_size = i32::from_str(label.strip_prefix("EBS-").unwrap()).unwrap();
        }
    }

    let run_instances = client
        .run_instances()
        .instance_type(instance_type)
        .max_count(1)
        .min_count(1)
        .set_block_device_mappings(Some(vec![BlockDeviceMapping::builder()
            .set_device_name(Some("/dev/sda1".into()))
            .set_ebs(Some(
                EbsBlockDevice::builder()
                    .set_delete_on_termination(Some(true))
                    .set_volume_size(Some(volume_size))
                    .set_volume_type(Some(VolumeType::Gp3))
                    .build(),
            ))
            .build()]))
        .set_launch_template(Some(
            LaunchTemplateSpecification::builder()
                .set_launch_template_id(Some(env::var("ARM64_LAUNCH_TEMPLATE_ID").unwrap()))
                .build(),
        ))
        .set_tag_specifications(Some(vec![TagSpecification::builder()
            .set_resource_type(Some(ResourceType::Instance))
            .set_tags(Some(vec![
                Tag::builder()
                    .set_key(Some("Name".into()))
                    .set_value(Some(format!(
                        "{}/{}/{}",
                        repository_full_name, workflow_job_id, workflow_run_id
                    )))
                    .build(),
                Tag::builder()
                    .set_key(Some("GitHubActionsRepository".into()))
                    .set_value(Some(repository_full_name.clone()))
                    .build(),
                Tag::builder()
                    .set_key(Some("GitHubActionsId".into()))
                    .set_value(Some(workflow_job_id))
                    .build(),
                Tag::builder()
                    .set_key(Some("GitHubActionsRunId".into()))
                    .set_value(Some(workflow_run_id))
                    .build(),
            ]))
            .build()]))
        .set_user_data(Some(user_data))
        .send()
        .await?;

    if run_instances.instances().is_empty() {
        panic!("No instances created.");
    }

    Ok(run_instances.instances.clone().unwrap()[0]
        .instance_id
        .clone()
        .unwrap())
}
