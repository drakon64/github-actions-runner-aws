resource "aws_vpc" "vpc" {
  cidr_block = "192.168.0.0/16"

  tags = {
    Name = "GitHubActionsRunner"
  }
}

resource "aws_subnet" "subnet" {
  vpc_id     = aws_vpc.vpc.id
  cidr_block = "192.168.0.0/16"

  map_public_ip_on_launch = true
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
}
