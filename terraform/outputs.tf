output "alb_dns_name" {
  value       = module.compute.alb_dns_name
  description = "The public DNS name of the ALB"
}

output "assets_bucket_name" {
  value       = module.storage.assets_bucket_name
  description = "The name of the assets S3 bucket"
}

output "backups_bucket_name" {
  value       = module.storage.backups_bucket_name
  description = "The name of the backups S3 bucket"
}

output "sns_topic_arn" {
  value       = module.monitoring.sns_topic_arn
  description = "The ARN of the SNS topic for alerts"
}
