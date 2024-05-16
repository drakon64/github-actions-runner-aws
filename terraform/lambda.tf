data "aws_iam_policy_document" "lambda_assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "lambda" {
  statement {
    actions       = ["sns:Publish"]
    effect        = "Allow"
    not_resources = ["arn:aws:sns:*:*:*"]
  }
}

resource "aws_iam_role" "lambda" {
  name               = "GitHubActionsRunner"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role_policy.json

  inline_policy {
    name   = "lambda"
    policy = data.aws_iam_policy_document.lambda.json
  }

  managed_policy_arns = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
}

data "local_file" "lambda" {
  filename = "${path.module}/../target/lambda/github-actions-runner-aws/bootstrap.zip"
}

resource "aws_lambda_function" "lambda" {
  function_name = "GitHubActionsRunner"
  role          = aws_iam_role.lambda.arn

  architectures = ["arm64"]

  #   environment {
  #     variables = {
  #       SECRET = var.webhook_secret
  #     }
  #   }

  filename         = data.local_file.lambda.filename
  handler          = "rust.handler"
  memory_size      = 128
  package_type     = "Zip"
  runtime          = "provided.al2023"
  source_code_hash = data.local_file.lambda.content_base64sha256
  timeout          = 3
}

resource "aws_lambda_permission" "lambda" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.arn
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.api_gateway.execution_arn}/*/"
}
