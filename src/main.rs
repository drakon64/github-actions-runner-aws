mod github;
mod run_instance;
mod webhook;

use crate::run_instance::run_instance;
use crate::webhook::{Action, Webhook};
use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<String, Error> {
    let webhook = serde_json::from_str::<Webhook>(&*event.payload.body.unwrap()).unwrap();
    let client = aws_sdk_ec2::Client::new(&aws_config::load_from_env().await);

    match webhook.action {
        Action::Queued => Ok(run_instance(client, webhook).await.unwrap()),
        Action::Completed => Ok("Completed".into()),
        _ => Ok("This webhooks runs only for `queued` and `completed` jobs".into()),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
