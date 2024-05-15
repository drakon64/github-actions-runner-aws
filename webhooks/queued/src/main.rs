use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Webhook {
    action: String,
    workflow_job: WorkflowJob,
    repository: Repository,
}

#[derive(Deserialize, Serialize)]
struct WorkflowJob {
    run_id: u64,
    run_attempt: usize,
}

#[derive(Deserialize, Serialize)]
struct Repository {
    full_name: String,
}

async fn function_handler(event: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<Webhook, Error> {
    let webhook = serde_json::from_str::<Webhook>(&*event.payload.body.unwrap()).unwrap();

    Ok(webhook)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
