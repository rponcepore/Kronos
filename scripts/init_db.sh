#!/usr/bin/env bash

# This script is meant just to fire up a postgres instance for testing
# Do NOT use this script for persistent postgres data

#First, make sure that we are in the correct directory:

EXPECTED_DIR="/home/$USER/Kronos/backend_kronos"
CURRENT_DIR="$(pwd)"

if [ "$CURRENT_DIR" == "$EXPECTED_DIR" ]; then
    echo "You are in the correct directory: $CURRENT_DIR"
else
    echo "Warning: You are in the wrong directory!"
    echo "Current directory: $CURRENT_DIR"
    echo "Expected directory: $EXPECTED_DIR"
    echo ""
    echo "Moving to correct directory."
    cd $EXPECTED_DIR
fi

# Make sure that sea-orm-cli is installed

if ! [ -x "$(command -v sea-orm-cli)" ]; then
  echo >&2 "Error: sea-orm-cli is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sea-orm-cli \
  --features sqlx-postgres, runtime-tokio-rustls"
  echo >&2 "to install it."
  exit 1
fi


# Make sure that the postgres-sql client is installed.
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  echo >&2 "Use:"
  echo >&2 "    sudo apt-get install postgresql-client   "
  echo >&2 "to install it."
  exit 1
fi

set -x 
set -eo pipefail 

# Check if custom parameter has been set, otherwise use default postgres values
# CHANGE IN PROD
DB_PORT="${POSTGRES_PORT:=5432}" 
SUPERUSER="${SUPERUSER:=postgres}" 
SUPERUSER_PWD="${SUPERUSER_PWD:=password}" 

APP_USER="${APP_USER:=app}" 
APP_USER_PWD="${APP_USER_PWD:=secret}" 
APP_DB_NAME="${APP_DB_NAME:=kronos_test}" 

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

# Note: This export only sets this env variable for the rest of the shel session, not permanently.
export DATABASE_URL=$DATABASE_URL

echo $APP_DB_NAME

#Create the application database inside the container.
OUTPUT=$(docker exec -it "${CONTAINER_NAME}" psql -U "${APP_USER}" postgres -c "CREATE DATABASE ${APP_DB_NAME};" 2>&1)
#psql -U "$APP_USER" -h "localhost" -p "$DB_PORT" -c "CREATE DATABASE $APP_DB_NAME;" 

# Check if an error occurred
if [[ "$OUTPUT" == *"ERROR:"* ]]; then
    echo "Error detected while creating database."

    # Check if the error is "database does not exist"
    if [[ "$OUTPUT" == *"database \"$APP_DB_NAME\" already exists"* ]]; then
        echo ""
        echo "Database '$APP_DB_NAME' already exists. No action needed."
    elif [[ "$OUTPUT" == *"database \"$APP_DB_NAME\" does not exist"* ]]; then
        echo ""
        echo "Database '$APP_DB_NAME' does not exist. You may need to create it manually."
    else
        echo ""
        echo "Unexpected error: $OUTPUT"
    fi
else
    echo "Database '$APP_DB_NAME' created successfully."
fi

echo "Applying migrations"

# We now are confident that the database exists.
# Apply all pending migrations
sea-orm-cli migrate up 