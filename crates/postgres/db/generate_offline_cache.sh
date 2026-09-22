#!/bin/bash

echo "Starting database"
# Relative path to the docker-compose.yml file from the script's location
COMPOSE_FILE_PATH="$(dirname "$0")"
echo "Using docker-compose file at: ${COMPOSE_FILE_PATH}"
docker compose --project-directory "${COMPOSE_FILE_PATH}" up --build --wait -d

echo "Waiting for database to be ready..."
# Wait for the database to be ready
sleep 10

echo "Ensuring sqlx-cli is installed"
cargo install sqlx-cli --no-default-features --features postgres,native-tls

echo "Generating offline cache"
cargo sqlx prepare --database-url "postgres://postgres:postgres@localhost:5432/postgres"

echo "Shutting down database"
docker compose --project-directory "${COMPOSE_FILE_PATH}" down -v
