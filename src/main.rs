use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize};

#[derive(Deserialize)]
enum Action {
    #[serde(rename = "waiting")]
    Waiting,

    #[serde(rename = "queued")]
    Queued,

    #[serde(rename = "in_progress")]
    InProgress,

    #[serde(rename = "completed")]
    Completed
}

#[derive(Deserialize)]
struct Webhook {
    action: Action,
    workflow_job: WorkflowJob,
    repository: Repository,
}

#[derive(Deserialize)]
struct WorkflowJob {
    run_id: u64,
    run_attempt: usize,
}

#[derive(Deserialize)]
struct Repository {
    full_name: String,
}

async fn function_handler(event: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<String, Error> {
    let webhook = serde_json::from_str::<Webhook>(&*event.payload.body.unwrap()).unwrap();

    match webhook.action {
        Action::Queued => { Ok("Queued".into()) }
        Action::Completed => { Ok("Completed".into()) }
        _ => { Ok("This webhooks runs only for `queued` and `completed` jobs".into()) }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
