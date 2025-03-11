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
    echo "Building environment."
else
    echo "Warning: You are in the wrong directory!"
    echo "Current directory: $CURRENT_DIR"
    echo "Expected directory: $EXPECTED_DIR"
    echo ""
    echo "Moving to correct directory."
    cd $EXPECTED_DIR
fi

chmod +x start_back.sh
chmod +x start_front.sh

INIT_DB_SCRIPT="../backend_kronos/scripts/init_db.sh"

# The frontend is located at port 9000, so there should be no collisions!
# make back can fail for lots of reasons. If it's something manageable, proceed.

# We're going to try to run docker. First, check if the docker daemon is running.
if ! docker info &> /dev/null
then
    echo "Docker is not running. Please start docker before running this script."
    echo "If you are a Windows user, start docker desktop and re-run this script."
    echo "For help, see: https://docs.docker.com/desktop/features/wsl/ "
    exit 1
fi

pwd

# Build the backend container in a new shell
wt.exe --window last new-tab wsl -e bash -i -c "../scripts/start_back.sh"
echo ""
echo "Building postgres container."
echo ""

# Start the frontend (locally) in a new shell
wt.exe --window last new-tab wsl -e bash -i -c "../scripts/start_front.sh"


# Open a browser window to the frontend.
cmd.exe /c start "http://localhost:9000"
echo ""
echo "Frontend running locally on port 9000."
echo ""

# Create a new database for testing.
echo "Running start_back.sh to create a new database for testing."
echo ""
./${INIT_DB_SCRIPT}



