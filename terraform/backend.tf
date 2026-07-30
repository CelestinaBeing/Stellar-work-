terraform {
  backend "s3" {
    bucket         = "stellarwork-terraform-state"
    key            = "state/terraform.tfstate"
    region         = "us-east-1"
    dynamodb_table = "stellarwork-terraform-locks"
    encrypt        = true
  }
}
