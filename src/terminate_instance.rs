use crate::webhook::Webhook;
use aws_sdk_ec2::error::SdkError;
use aws_sdk_ec2::operation::terminate_instances::TerminateInstancesError;
use aws_sdk_ec2::Client;

pub(crate) async fn terminate_instance(
    client: Client,
    webhook: Webhook,
) -> Result<String, SdkError<TerminateInstancesError>> {
    let runner_name = webhook.workflow_job.runner_name.unwrap();

    client
        .terminate_instances()
        .set_instance_ids(Some(vec![runner_name.clone()]))
        .send()
        .await?;

    Ok(format!("Terminated instance {}", runner_name))
}
