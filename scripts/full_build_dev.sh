#!/bin/bash

# This script sets up a non-database enabled testing environment with docker desktop. 
# It is meant to be executed from the "scripts" directory

# This script sets up just the docker container for the backend, exposing it on the correct ports.
# The server is located at port 8000.

# This script has been updated for new users running WSL.
# check if rust is installed. 
if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed. Please install rust before running this script."
    echo "You can install rust by running the following command:"
    echo "curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh" #This works for linux and max
    exit 1
else
  rustup update
fi

#check if make is installed
if ! command -v make &> /dev/null
then
    echo "Make is not installed. Please install make before running this script."
    echo "You can install make by running the following command:"
    echo "sudo apt-get install make" #This works for linux. I don't know about mac. 
    exit 1
fi

#Now, make sure that we are in the correct directory:

EXPECTED_DIR="/home/$USER/Kronos/scripts"
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

cd ..

# The frontend is located at port 9000, so there should be no collisions!
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

# We're going to try to run docker. First, check if the docker daemon is running.
if ! docker info &> /dev/null
then
    echo "Docker is not running. Please start docker before running this script."
    echo "If you are a Windows user, start docker desktop and re-run this script."
    echo "For help, see: https://docs.docker.com/desktop/features/wsl/ "
    exit 1
fi



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
echo "Backend container built. Running."
echo "To run the frontend container, in a separate shell, run \"npm run dev\" from the frontend_kronos directory."

docker run -p 8000:8000 backend_kronos 