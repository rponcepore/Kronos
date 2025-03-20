#!/bin/bash

# This script populates the database with synthetic data.

# Set env variable
# Move to the correct directory
EXPECTED_DIR="/home/$USER/Kronos/backend_kronos"

cd $EXPECTED_DIR

# Read the config file to get teh database URL
CONFIG_FILE="configs/backend_configuration.yaml"

echo "Reading from config file: ${CONFIG_FILE}"

# Function to read YAML values with fallback to default
read_config() {
    local key="$1"
    local default="$2"
    local value
    value=$(yq e "$key // \"$default\"" "$CONFIG_FILE" 2>/dev/null)
    echo "$value"
}

# Read values from the config file or use defaults
DB_PORT=$(read_config '.database.port' "5432")
SUPERUSER=$(read_config '.database.superuser' "postgres")
SUPERUSER_PWD=$(read_config '.database.superuser_pwd' "password")

APP_USER=$(read_config '.database.app_user' "app")
APP_USER_PWD=$(read_config '.database.app_user_pwd' "password")
APP_DB_NAME=$(read_config '.database.app_db_name' "kronos_db")

DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
echo "read url as: $DATABASE_URL"

export DATABASE_URL

echo "env variable \"DATABASE_URL\" set to:"

# Wait for Postgres to be ready to accept connections
CONTAINER_NAME="postgres"

  until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
  ]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1  
  done 

# Connect to the database
STATEMENT=""
docker exec -it "${CONTAINER_NAME}" psql -U "${APP_USER}" -c "${STATEMENT}"

# Begin executing queries