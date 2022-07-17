AUTH_ECR_REPO=$(terraform output -raw auth_repository_url)

aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin $AUTH_ECR_REPO
docker build -t ostrich_auth_repository ../.
docker tag ostrich_auth_repository:latest $AUTH_ECR_REPO/ostrich_auth_repository:latest
docker push $AUTH_ECR_REPO/ostrich_auth_repository:latest