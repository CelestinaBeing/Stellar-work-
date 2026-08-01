variable "environment" {
  type        = string
  description = "Environment name (dev, staging, prod)"
}

variable "vpc_id" {
  type        = string
  description = "VPC ID"
}

variable "public_subnet_ids" {
  type        = list(string)
  description = "List of public subnet IDs"
}

variable "private_subnet_ids" {
  type        = list(string)
  description = "List of private subnet IDs"
}

variable "alb_security_group_id" {
  type        = string
  description = "Security group ID for the ALB"
}

variable "ecs_tasks_security_group_id" {
  type        = string
  description = "Security group ID for ECS tasks"
}

variable "container_image" {
  type        = string
  description = "Docker image for the Next.js frontend"
  default     = "stellarwork-frontend:latest"
}

variable "container_port" {
  type        = number
  description = "Port exposed by the container"
  default     = 3000
}

variable "cpu" {
  type        = string
  description = "Fargate instance CPU units"
  default     = "256"
}

variable "memory" {
  type        = string
  description = "Fargate instance memory (MiB)"
  default     = "512"
}

variable "app_count" {
  type        = number
  description = "Number of ECS tasks to run"
  default     = 2
}
