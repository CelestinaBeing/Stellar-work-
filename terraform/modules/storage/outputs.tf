output "assets_bucket_name" {
  value       = aws_s3_bucket.assets.id
  description = "The name of the assets S3 bucket"
}

output "assets_bucket_arn" {
  value       = aws_s3_bucket.assets.arn
  description = "The ARN of the assets S3 bucket"
}

output "backups_bucket_name" {
  value       = aws_s3_bucket.backups.id
  description = "The name of the backups S3 bucket"
}
