use lambda_runtime::{run, service_fn, tracing, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {}

async fn function_handler(event: Request) -> Result<Response, Error> {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
