#!/bin/bash

#First, orient the user:
echo "This is the BACKEND tab." 

# Directory check

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

# Build
docker build --tag backend_kronos . 2>&1 # Uses the dockerfile in the backend_kronos directory
dock_ret_val=$?  # Capture the actual exit code of docker build


echo "$docker_output"

# Check if the specific error message is in the output
if echo "$docker_output" | grep -q "The command 'docker-compose' could not be found in this WSL 2 distro."; then
  echo "ERROR: Docker Compose is not installed or not available in this WSL 2 environment."
  echo ""
  echo "FIX: If you installed docker desktop on windows, it needs to be running to support docker compose in WSL."
  echo "FIX: Run docker desktop, then re-run this script."
  exit 1
fi

# Handle other potential failures
if [[ $dock_ret_val -ne 0 ]]; then
  echo "ERROR: Docker Compose command failed with exit code $dock_ret_val."
  exit $dock_ret_val
fi

# Container built
echo "Backend container built."

docker run -p 8000:8000 backend_kronos