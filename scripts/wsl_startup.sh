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

# Get the project root directory (parent of scripts directory)
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "PROJECT ROOT: ${PROJECT_ROOT}"

# Important directories:
DATABASE_DIR="${PROJECT_ROOT}/database_kronos"
BACKEND_DIR="${PROJECT_ROOT}/backend_kronos"
FRONTEND_DIR="${PROJECT_ROOT}/frontend_kronos"

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
    brew install zstd
    export PKG_CONFIG_PATH="/opt/homebrew/lib/pkgconfig"
  else
    echo >&2 "    sudo apt-get install postgresql-client"
  fi
  echo >&2 "to install it."
  exit 1
fi

#set -x 
set -eo pipefail 

# Now we need to run our initdb script.
cd "${DATABASE_DIR}"
source "./scripts/init_db.sh"

# Database should be ready to go. 
cd "${BACKEND_DIR}"

# Generate the new entities. 
sea-orm-cli generate entity \
        -u postgres://postgres:password@localhost:5432/kronos_db \
        -o src/models/entities \
        --with-serde both

cargo test

#If all tests pass, can start up the backend
echo "Startup successful!"

echo "Start the backend in this or a separate shell with \"cargo run\" in Kronos/backend_kronos"
echo "Then test the frontend with \"npm run test\" in Kronos/frontend_kronos"
