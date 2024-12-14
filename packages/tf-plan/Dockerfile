# Сборка образа
docker build -t tf-plan -f packages/tf-plan/Dockerfile .

# Запуск
docker run -v $(pwd):/workspace \
  -e AWS_ACCESS_KEY_ID=xxx \
  -e AWS_SECRET_ACCESS_KEY=xxx \
  tf-plan --working-dir /workspace