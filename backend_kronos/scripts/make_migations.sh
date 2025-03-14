#!/bin/bash

# Move to the correct directory
EXPECTED_DIR="/home/$USER/Kronos/backend_kronos"

cd $EXPECTED_DIR

# Read the config file to get teh database URL
CONFIG_FILE="backend_configuration.yaml"

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

# Then do the migrations

export DATABASE_URL

sea-orm-cli migrate up