#!/usr/bin/env bash

# This script is meant just to fire up a postgres instance for testing
# Do NOT use this script for persistent postgres data

#It's also possible that this script is being executed when a postgres instance is already available.
# In that case, we don't want to run anything.

if docker ps --filter "name=postgres" --format "{{.Names}}" | grep -q "^postgres$"; then
    echo "PostgreSQL container is already running. Exiting script."
    exit 0
else
    echo "No running PostgreSQL container found. Continuing script..."
    # Start your PostgreSQL container or run the rest of your script
fi

# Get the project root directory (parent of database_kronos directory)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DATABASE_DIR="${PROJECT_ROOT}/database_kronos"

cd "${DATABASE_DIR}"

# Make sure that sea-orm-cli is installed
if ! [ -x "$(command -v sea-orm-cli)" ]; then
  echo >&2 "Error: sea-orm-cli is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sea-orm-cli"
  echo >&2 "to install it."
  exit 1
fi

# Make sure that the postgres-sql client is installed.
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  echo >&2 "Use:"
  if [[ "$OSTYPE" == "darwin"* ]]; then
    echo >&2 "    brew install postgresql@14"
  else
    echo >&2 "    sudo apt-get install postgresql-client"
  fi
  echo >&2 "to install it."
  exit 1
fi

#set -x 
set -eo pipefail 

# Ensure `yq` is installed
if ! command -v yq &>/dev/null; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Error: yq is not installed. Install it with 'brew install yq'"
    else
        echo "Error: yq is not installed. Install it with 'sudo snap install yq'"
    fi
    exit 1
fi

# Check if custom parameter has been set, otherwise use default postgres values
# These need to be read in from backend_configuration.yaml

CONFIG_FILE="../configs/backend_configuration.yaml"
pwd
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

# Print values (for debugging)
echo "DB_PORT=${DB_PORT}"
echo "SUPERUSER=${SUPERUSER}"
echo "SUPERUSER_PWD=${SUPERUSER_PWD}"
echo "APP_USER=${APP_USER}"
echo "APP_USER_PWD=${APP_USER_PWD}"
echo "APP_DB_NAME=${APP_DB_NAME}"

# DB_PORT="${POSTGRES_PORT:=5432}" 
# SUPERUSER="${SUPERUSER:=postgres}" 
# SUPERUSER_PWD="${SUPERUSER_PWD:=password}" 

# APP_USER="${APP_USER:=app}" 
# APP_USER_PWD="${APP_USER_PWD:=secret}" 
# APP_DB_NAME="${APP_DB_NAME:=kronos_db}" 

# We are about to spin up a new docker container. Skip this step if one is already running.
# We only know if one is running if we set the env variable SKIP_DOCKER in a previous instantiation.
if [[ -z "${SKIP_DOCKER}" ]]
then
  # Pull the latest image
  docker pull postgres

  #Turn off old container, so we can run this one
  docker kill postgres || true
  docker remove postgres || true

  # Launch postgres using Docker
  CONTAINER_NAME="postgres"
  docker run \
    --env POSTGRES_PASSWORD=${SUPERUSER_PWD}  \
    --health-cmd="pg_isready -U ${SUPERUSER} || exit 1"  \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5  \
    --publish "${DB_PORT}":5432 \
    --detach \
    --name "${CONTAINER_NAME}" \
    postgres -N 1000  
    # ^ Increased maximum number of connections for testing purposes
    # removed for reasons beyond my imagination:  --env POSTGRES_USER=${SUPERUSER}  \

  # Wait for Postgres to be ready to accept connections
  until [ \
    "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
    "healthy" \
  ]; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1  
  done 

  CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

  # Grant create db privileges to the app user
  GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"

  # Grant public schema rights to the user
  GRANT_USAGE="GRANT USAGE ON SCHEMA public TO ${APP_USER}"
  docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_USAGE}"
fi

  echo "Postgres is up and running on port ${DB_PORT}. Running migrations now." 

#Export the DATABASE_URL environmental variable
DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}

# Note: This export only sets this env variable for the rest of the shell session, not permanently.
export DATABASE_URL

echo "CREATING THE APPLICATION DATABASE: ${APP_DB_NAME}"

#Create the application database inside the container.
OUTPUT=$(docker exec -it "${CONTAINER_NAME}" psql -U "${APP_USER}" postgres -c "CREATE DATABASE $APP_DB_NAME;" 2>&1)

# Check if an error occurred
if [[ "$OUTPUT" == *"ERROR:"* ]]; then
    echo "Error detected while creating database."

    # Check if the error is "database does not exist"
    if [[ "$OUTPUT" == *"database \"$APP_DB_NAME\" already exists"* ]]; then
        echo ""
        echo "Database '$APP_DB_NAME' already exists. No action needed."
        exit 1
    elif [[ "$OUTPUT" == *"database \"$APP_DB_NAME\" does not exist"* ]]; then
        echo ""
        echo "Database '$APP_DB_NAME' does not exist. You may need to create it manually."
        exit 1
    else
        echo ""
        echo "Unexpected error: $OUTPUT"
        exit 1
    fi
else
    echo "Database '$APP_DB_NAME' created successfully."
fi

echo "Applying migrations"

# We now are confident that the database exists.
# Apply all pending migrations
sea-orm-cli migrate up 
