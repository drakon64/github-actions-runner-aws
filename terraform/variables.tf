variable "bucket" {
  type      = string
  sensitive = true
}

variable "object" {
  type    = string
  default = "bootstrap.zip"
}

variable "region" {
  type = string
}

variable "prefix" {
  type    = string
  default = ""
}

variable "suffix" {
  type    = string
  default = ""
}

variable "client_id" {
  type      = string
  sensitive = true
}

variable "private_key" {
  type      = string
  sensitive = true
}

variable "secret_token" {
  type      = string
  sensitive = true
}

variable "grafana_cloud_stack_name" {
  type      = string
  sensitive = true
}

variable "grafana_cloud_token" {
  type      = string
  sensitive = true
}
