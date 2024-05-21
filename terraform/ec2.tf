data "aws_iam_policy_document" "ec2_assume_role_policy" {
  statement {
    actions = ["sts:AssumeRole"]
    effect  = "Allow"

    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
  }
}

data "aws_iam_policy_document" "ec2" {
  statement {
    actions   = ["ec2:CreateTags"]
    effect    = "Allow"
    resources = ["*"]
  }
}

resource "aws_iam_role" "ec2" {
  name               = "${var.prefix}GitHubActionsRunnerEC2${var.suffix}"
  assume_role_policy = data.aws_iam_policy_document.ec2_assume_role_policy.json

  inline_policy {
    name   = "GitHubActionsRunner"
    policy = data.aws_iam_policy_document.ec2.json
  }
}

resource "aws_iam_instance_profile" "ec2" {
  name = "${var.prefix}GitHubActionsRunner${var.suffix}"
  role = aws_iam_role.ec2.name
}

locals {
  architectures = toset(["arm64", "x86_64"])
}

data "aws_ami" "ubuntu" {
  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-*-server-*"]
  }

  filter {
    name   = "architecture"
    values = [each.value]
  }

  filter {
    name   = "root-device-type"
    values = ["ebs"]
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  most_recent = true
  owners      = ["099720109477"]

  for_each = local.architectures
}

resource "aws_launch_template" "ubuntu" {
  ebs_optimized = true

  iam_instance_profile {
    name = aws_iam_instance_profile.ec2.name
  }

  image_id = data.aws_ami.ubuntu[each.value].id

  network_interfaces {
    delete_on_termination = true
    subnet_id             = aws_subnet.subnet.id
  }

  update_default_version = true

  for_each = local.architectures
}
