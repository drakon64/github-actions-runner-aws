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
