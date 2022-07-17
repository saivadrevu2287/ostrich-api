output "auth_repository_url" {
  description = "URL of the Auth ECR Repository"
  value       = aws_ecr_repository.auth_repository.repository_url
}