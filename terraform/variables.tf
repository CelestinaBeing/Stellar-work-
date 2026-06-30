variable "aws_region" {
  type        = string
  description = "AWS Region to deploy resources"
  default     = "us-east-1"
}

variable "environment" {
  type        = string
  description = "Environment name (dev, staging, prod)"
}

variable "vpc_cidr" {
  type        = string
  description = "VPC CIDR block"
  default     = "10.0.0.0/16"
}

variable "container_image" {
  type        = string
  description = "Docker image for the Next.js frontend"
  default     = "stellarwork-frontend:latest"
}

variable "app_count" {
  type        = number
  description = "Number of ECS tasks to run"
  default     = 2
}

variable "email_recipient" {
  type        = string
  description = "Email address to receive alerts"
  default     = "alerts@stellarwork.io"
}
