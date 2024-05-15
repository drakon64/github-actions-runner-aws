use aws_lambda_events::apigw::ApiGatewayV2httpRequest;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<ApiGatewayV2httpRequest>) -> Result<String, Error> {
    let body = event.payload.body.unwrap();

    println!("{}", body);

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
