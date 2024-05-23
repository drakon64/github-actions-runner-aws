mod github;
mod run_instance;
mod terminate_instance;
mod webhook;

use crate::run_instance::run_instance;
use crate::terminate_instance::terminate_instance;
use crate::webhook::{Action, Webhook};
use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use hmac::{Hmac, Mac};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use sha2::Sha256;
use std::env;

async fn function_handler(event: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<String, Error> {
    let body = event.payload.body.unwrap();
    let webhook = serde_json::from_str::<Webhook>(&*body).unwrap();

    // TODO: Ugly hack, remove this in your own deployments
    if !(webhook.repository.owner.login == "drakon64"
        || webhook.repository.owner.login == "lilyinstarlight")
    {
        panic!(
            "Unauthorised repository owner: {}",
            webhook.repository.owner.login
        )
    }

    Hmac::<Sha256>::new_from_slice(env::var("SECRET_TOKEN").unwrap().as_bytes())
        .unwrap()
        .chain_update(body.as_bytes())
        .verify_slice(
            hex::decode(
                &event.payload.headers["X-Hub-Signature-256"]
                    .to_str()
                    .unwrap()
                    .strip_prefix("sha256=")
                    .unwrap(),
            )
            .unwrap()
            .as_slice(),
        )
        .unwrap();

    if !webhook
        .workflow_job
        .labels
        .contains(&"drakon64/github-actions-runner-aws".to_string())
    {
        return Ok("EC2 runner not requested.".into()); // TODO: Return a HTTP status of no action (or no content)
    }

    let client = aws_sdk_ec2::Client::new(&aws_config::load_from_env().await);

    match webhook.action {
        Action::Queued => Ok(run_instance(client, webhook).await.unwrap()),
        Action::Completed => Ok(terminate_instance(client, webhook).await.unwrap()),
        _ => Ok("This webhooks runs only for `queued` and `completed` jobs".into()),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
