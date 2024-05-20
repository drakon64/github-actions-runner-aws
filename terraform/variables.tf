variable "region" {
  type = string
}

variable "access_key" {
  type      = string
  sensitive = true
}

variable "secret_access_key" {
  type      = string
  sensitive = true
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
