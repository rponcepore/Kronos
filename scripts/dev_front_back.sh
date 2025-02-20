#!/bin/bash

# This script sets up a non-database enabled testing environment with docker desktop. 
# It is meant to be executed from the "scripts" directory

# This script sets up just the docker container for the backend, exposing it on the correct ports.
# The server is located at port 8000.

# The frontend is located at port 9000, so there should be no collisions!

cd ..

# make back can fail for lots of reasons. If it's something manageable, proceed.
make back; retval=$?
echo "retval = $retval"  ## now, adding more logging won't break the logic.
case $retval in
  0) 
    echo "Make back successful"
    ;;
  *)   
    echo "Backend construction failed. Would you like to proceed anyway?" 
    read -p "Enter [Y]/[n]: " answer

    if [[ "$answer" =~ ^[Yy]$ || -z "$answer" ]]; then
      echo "Proceeding..."
      
    else
      echo "Aborting..."
      exit 1
    fi
    ;;
esac

cd backend_kronos
cargo build
echo "Target built. Building container."

# Build
docker build --tag backend_kronos . 2>&1 
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

#Container built
echo "Container built. Running."
echo ""

docker run -p 8000:8000 backend_kronos 