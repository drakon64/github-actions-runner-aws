use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::http::HeaderMap;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

async fn function_handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = event.payload.body.unwrap();
    let headers = HeaderMap::new();

    println!("{}", body);

    Ok(ApiGatewayProxyResponse {
        body: Some(body.into()),
        headers: headers.clone(),
        is_base64_encoded: false,
        multi_value_headers: headers,
        status_code: 200,
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
