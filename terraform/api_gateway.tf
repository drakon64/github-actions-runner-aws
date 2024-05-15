resource "aws_apigatewayv2_api" "api_gateway" {
  name = "GitHubActionsRunner"

  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "api_gateway" {
  api_id = aws_apigatewayv2_api.api_gateway.id
  name   = "$default"

  auto_deploy = true
}

resource "aws_apigatewayv2_integration" "api_gateway" {
  api_id           = aws_apigatewayv2_api.api_gateway.id
  integration_type = "AWS_PROXY"

  integration_method     = "POST"
  integration_uri        = aws_lambda_function.lambda.arn
  payload_format_version = "2.0"
}

resource "aws_apigatewayv2_route" "api_gateway" {
  api_id    = aws_apigatewayv2_api.api_gateway.id
  route_key = "POST /"

  target = "integrations/${aws_apigatewayv2_integration.api_gateway.id}"
}

output "payload_url" {
  value = aws_apigatewayv2_api.api_gateway.api_endpoint
}
