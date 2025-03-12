#!/bin/bash

#First, orient the user:
echo "This is the BACKEND tab." 

# Directory check

EXPECTED_DIR="/home/$USER/Kronos/backend_kronos"
CURRENT_DIR="$(pwd)"

if [ "$CURRENT_DIR" == "$EXPECTED_DIR" ]; then
    echo "You are in the correct directory: $CURRENT_DIR"
else
    # echo "Warning: You are in the wrong directory!"
    # echo "Current directory: $CURRENT_DIR"
    # echo "Expected directory: $EXPECTED_DIR"
    echo ""
    echo "Moving to correct directory."
    cd $EXPECTED_DIR
fi

# Build
cargo run