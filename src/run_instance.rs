use crate::webhook::Webhook;
use aws_sdk_ec2::types::{
    BlockDeviceMapping, EbsBlockDevice, ResourceType, Tag, TagSpecification, VolumeType,
};
use aws_sdk_ec2::{Client, Error};

pub(crate) async fn run_instance(client: Client, webhook: Webhook) -> Result<String, Error> {
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
                    .set_key(Some("GitHubActionsRepository".into()))
                    .set_value(Some(webhook.repository.full_name))
                    .build(),
                Tag::builder()
                    .set_key(Some("GitHubActionsId".into()))
                    .set_value(Some(webhook.workflow_job.id.to_string()))
                    .build(),
                Tag::builder()
                    .set_key(Some("GitHubActionsRunId".into()))
                    .set_value(Some(webhook.workflow_job.run_id.to_string()))
                    .build(),
            ]))
            .build()]))
        .set_subnet_id(Some(std::env::var("SUBNET").unwrap()))
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
