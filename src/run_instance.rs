use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use aws_sdk_ec2::types::{
    BlockDeviceMapping, EbsBlockDevice, InstanceInterruptionBehavior, InstanceMarketOptionsRequest,
    InstanceType, LaunchTemplateSpecification, MarketType, ResourceType, SpotInstanceType,
    SpotMarketOptions, Tag, TagSpecification, VolumeType,
};
use aws_sdk_ec2::{Client, Error};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;
use std::str::FromStr;

pub(crate) async fn run_instance(client: Client, webhook: Webhook) -> Result<String, Error> {
    let repository_full_name = &webhook.repository.full_name;

    let mut instance_type = InstanceType::M7gLarge;
    let mut launch_template_variable = "ARM64_LAUNCH_TEMPLATE_ID";
    let mut volume_size: i32 = 14; // This can fit in an u16
    let mut spot: Option<MarketType> = None;
    let mut spot_options: Option<SpotMarketOptions> = None;

    for label in &webhook.workflow_job.labels {
        if label == "X64" {
            launch_template_variable = "X64_LAUNCH_TEMPLATE_ID";
        } else if label.starts_with("EC2-") && label != "EC2-Spot" {
            instance_type = InstanceType::from_str(label.strip_prefix("EC2-").unwrap()).unwrap();
        } else if label.starts_with("EBS-") && label.ends_with("GB") {
            volume_size = i32::from_str(
                label
                    .strip_prefix("EBS-")
                    .unwrap()
                    .strip_suffix("GB")
                    .unwrap(),
            )
            .unwrap();
        } else if label == "EC2-Spot" {
            spot = Some(MarketType::Spot);
            spot_options = Some(
                SpotMarketOptions::builder()
                    .set_instance_interruption_behavior(Some(
                        InstanceInterruptionBehavior::Terminate,
                    ))
                    .set_spot_instance_type(Some(SpotInstanceType::OneTime))
                    .build(),
            );
        }
    }

    // TODO: Get cloud-init to do this
    let user_data = BASE64_STANDARD.encode(format!("#!/bin/sh

sysctl vm.swappiness=1
mkswap /dev/nvme1n1
swapon /dev/nvme1n1

add-apt-repository ppa:ansible/ansible # https://github.com/ansible/ansible/issues/77624
apt-get update
apt-get -y install ansible-core awscli
apt-get clean
ansible-galaxy collection install amazon.aws community.general
ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --extra-vars 'url=https://github.com/{}' --extra-vars 'token={}' --extra-vars 'ebs_volume_size={}' ansible/runner.yml"
    , &repository_full_name, create_registration_token_for_repository(&repository_full_name, &webhook), volume_size));

    let run_instances = client
        .run_instances()
        .instance_type(instance_type)
        .max_count(1)
        .min_count(1)
        .set_block_device_mappings(Some(vec![
            BlockDeviceMapping::builder()
                .set_device_name(Some("/dev/sda1".into()))
                .set_ebs(Some(
                    EbsBlockDevice::builder()
                        .set_delete_on_termination(Some(true))
                        .set_iops(Some(3000))
                        .set_throughput(Some(128))
                        .set_volume_size(Some(volume_size))
                        .set_volume_type(Some(VolumeType::Gp3))
                        .build(),
                ))
                .build(),
            BlockDeviceMapping::builder()
                .set_device_name(Some("/dev/sdb".into()))
                .set_ebs(Some(
                    EbsBlockDevice::builder()
                        .set_delete_on_termination(Some(true))
                        .set_iops(Some(3000))
                        .set_throughput(Some(128))
                        .set_volume_size(Some(16))
                        .set_volume_type(Some(VolumeType::Gp3))
                        .build(),
                ))
                .build(),
        ]))
        .set_instance_market_options(Some(
            InstanceMarketOptionsRequest::builder()
                .set_market_type(spot)
                .set_spot_options(spot_options)
                .build(),
        ))
        .set_launch_template(Some(
            LaunchTemplateSpecification::builder()
                .set_launch_template_id(Some(env::var(launch_template_variable).unwrap()))
                .build(),
        ))
        .set_tag_specifications(Some(vec![TagSpecification::builder()
            .set_resource_type(Some(ResourceType::Instance))
            .set_tags(Some(vec![
                Tag::builder()
                    .set_key(Some("Name".into()))
                    .set_value(Some(repository_full_name.clone()))
                    .build(),
                Tag::builder()
                    .set_key(Some("GitHubActionsRepository".into()))
                    .set_value(Some(repository_full_name.clone()))
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
