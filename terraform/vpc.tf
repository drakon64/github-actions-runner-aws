data "aws_availability_zones" "availability_zones" {
  filter {
    name   = "opt-in-status"
    values = ["opt-in-not-required"]
  }
}

resource "aws_vpc" "vpc" {
  cidr_block = "192.168.0.0/16"

  tags = {
    Name = "${var.prefix}GitHubActionsRunner${var.suffix}"
  }
}

resource "aws_subnet" "subnet" {
  vpc_id     = aws_vpc.vpc.id
  cidr_block = "192.168.${count.index}.0/24"

  availability_zone                   = data.aws_availability_zones.availability_zones.names[count.index]
  map_public_ip_on_launch             = true
  private_dns_hostname_type_on_launch = "resource-name"

  count = length(data.aws_availability_zones.availability_zones.names)
}

resource "aws_internet_gateway" "igw" {
  vpc_id = aws_vpc.vpc.id
}

resource "aws_default_route_table" "route_table" {
  default_route_table_id = aws_vpc.vpc.default_route_table_id

  route {
    cidr_block = "0.0.0.0/0"
    gateway_id = aws_internet_gateway.igw.id
  }
}

resource "aws_default_network_acl" "acl" {
  default_network_acl_id = aws_vpc.vpc.default_network_acl_id

  egress {
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
    protocol   = "all"
    rule_no    = 1
  }

  ingress {
    action     = "allow"
    cidr_block = "0.0.0.0/0"
    from_port  = 0
    to_port    = 0
    protocol   = "all"
    rule_no    = 1
  }

  subnet_ids = [aws_subnet.subnet[0].id, aws_subnet.subnet[1].id, aws_subnet.subnet[2].id] # TODO: Fix this
}
