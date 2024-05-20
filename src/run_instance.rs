use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use aws_sdk_ec2::types::{
    BlockDeviceMapping, EbsBlockDevice, ResourceType, Tag, TagSpecification, VolumeType,
};
use aws_sdk_ec2::{Client, Error};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

pub(crate) async fn run_instance(client: Client, webhook: Webhook) -> Result<String, Error> {
    let repository_full_name = webhook.repository.full_name;
    let workflow_job_id = webhook.workflow_job.id.to_string();
    let workflow_run_id = webhook.workflow_job.run_id.to_string();

    // TODO: Get cloud-init to do this
    let user_data = BASE64_STANDARD.encode(format!("#!/bin/sh

add-apt-repository ppa:ansible/ansible # https://github.com/ansible/ansible/issues/77624
apt-get update
apt-get -y install ansible-core
ansible-galaxy collection install amazon.aws community.general
ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --extra-vars 'url=https://github.com/{}' --extra-vars 'token={}' ansible/runner.yml"
    , repository_full_name, create_registration_token_for_repository(&repository_full_name)));

    let run_instances = client
        .run_instances()
        .image_id("ami-012516325fcc21ec8")
        .instance_type(aws_sdk_ec2::types::InstanceType::R7gLarge)
        .set_block_device_mappings(Some(vec![BlockDeviceMapping::builder()
            .set_device_name(Some("/dev/sda1".into()))
            .set_ebs(Some(
                EbsBlockDevice::builder()
                    .set_delete_on_termination(Some(true))
                    .set_snapshot_id(Some("snap-0235e2397591fdc6f".into()))
                    .set_volume_size(Some(14))
                    .set_volume_type(Some(VolumeType::Gp3))
                    .build(),
            ))
            .build()]))
        .set_ebs_optimized(Some(true))
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
                    .set_value(Some(repository_full_name))
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
        .set_subnet_id(Some(std::env::var("SUBNET").unwrap()))
        .set_user_data(Some(user_data))
        .min_count(1)
        .max_count(1)
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
