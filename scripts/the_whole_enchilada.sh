#!/bin/bash
echo ""
echo "This script should be run once, successfully. It sets up a full environment."
echo ""
echo "WARNING: Running this script will create a new database container, every time."
echo "If you need to run it multiple times, for whatever reason (i.e., you have irreperably broken something) \
then you should delete the postgres container and then rerun the script."
echo ""
echo "This script assumes that you have installed Rust, npm, react, etc."
echo "This script also assumes that your docker daemon is running. Most commonly this is done by running Docker Desktop if on Windows."

# Important directories:
EXPECTED_DIR="/home/$USER/Kronos/scripts"
MIGRATION_DIR="/home/$USER/Kronos/backend_kronos/migration"
BACKEND_DIR="/home/$USER/Kronos/backend_kronos"
FRONTEND_DIR="/home/$USER/Kronos/frontend_kronos"
cd ${EXPECTED_DIR}

echo ""
echo "First, we will confirm that your docker daemon has started."
echo ""
# Check to make sure docker daemon is started. If not, exit and print scolding message.
# We're going to try to run docker. First, check if the docker daemon is running.
if ! docker info &> /dev/null
then
    echo "Docker is not running. Please start docker before running this script."
    echo "If you are a Windows user, start docker desktop and re-run this script."
    echo "For help, see: https://docs.docker.com/desktop/features/wsl/ "
    exit 1
fi

# Now, check for necessary libraries.
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

#set -x 
set -eo pipefail 

# Now we need to run our initdb script.
cd ${BACKEND_DIR}
source "./scripts/init_db.sh"

# Now we need to run database migrations to populate the database
cd ${MIGRATION_DIR}

cargo run

# Database should be ready to go. 
cd ${BACKEND_DIR}

cargo test

#If all tests pass, can start up the backend
echo "Startup successful!"

echo "Start the backend in this or a separate shell with \"cargo run\" in Kronos/backend_kronos"
echo "Then test the frontend with \"npm run test\" in Kronos/frontend_kronos"
