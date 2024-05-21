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
    actions = [
      "ec2:RunInstances",
      "ec2:CreateTags",
      "iam:PassRole",
      "ssm:GetParameters",
      "ec2:TerminateInstances",
    ]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_role" "lambda" {
  name               = "${var.prefix}GitHubActionsRunner${var.suffix}"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role_policy.json

  inline_policy {
    name   = "GitHubActionsRunner"
    policy = data.aws_iam_policy_document.lambda.json
  }

  managed_policy_arns = ["arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"]
}

data "aws_s3_object" "lambda" {
  bucket = "drakon64-github-actions-runner-aws"
  key    = "bootstrap.zip"
}

resource "aws_lambda_function" "lambda" {
  function_name = "${var.prefix}GitHubActionsRunner${var.suffix}"
  role          = aws_iam_role.lambda.arn

  architectures = ["arm64"]

  environment {
    variables = {
      ARM64_LAUNCH_TEMPLATE_ID = aws_launch_template.ubuntu["arm64"].id
      CLIENT_ID                = var.client_id
      PRIVATE_KEY              = var.private_key
      SECRET_TOKEN             = var.secret_token
    }
  }

  handler          = "rust.handler"
  memory_size      = 128
  package_type     = "Zip"
  runtime          = "provided.al2023"
  s3_bucket        = data.aws_s3_object.lambda.bucket
  s3_key           = data.aws_s3_object.lambda.key
  source_code_hash = data.aws_s3_object.lambda.checksum_sha256
  timeout          = 3
}

resource "aws_lambda_permission" "lambda" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.lambda.arn
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.api_gateway.execution_arn}/*/"
}
