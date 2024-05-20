use crate::webhook::Webhook;
use aws_sdk_ec2::error::SdkError;
use aws_sdk_ec2::operation::terminate_instances::{
    TerminateInstancesError, TerminateInstancesOutput,
};
use aws_sdk_ec2::Client;

pub(crate) async fn terminate_instance(
    client: Client,
    webhook: Webhook,
) -> Result<TerminateInstancesOutput, SdkError<TerminateInstancesError>> {
    client
        .terminate_instances()
        .set_instance_ids(Some(vec![webhook.workflow_job.runner_name]))
        .send()
}
