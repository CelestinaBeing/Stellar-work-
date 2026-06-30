output "alb_dns_name" {
  value       = aws_lb.main.dns_name
  description = "The public DNS name of the ALB"
}

output "ecs_cluster_name" {
  value       = aws_ecs_cluster.main.name
  description = "The name of the ECS cluster"
}

output "ecs_service_name" {
  value       = aws_ecs_service.main.name
  description = "The name of the ECS service"
}
