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
  image_id      = data.aws_ami.ubuntu[each.value].id

  network_interfaces {
    delete_on_termination = true
    subnet_id             = aws_subnet.subnet.id
  }

  update_default_version = true

  for_each = local.architectures
}
