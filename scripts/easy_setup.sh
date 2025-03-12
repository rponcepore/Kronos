#!/bin/bash

echo "Starting build process."

# Move to the correct directory
EXPECTED_DIR="/home/$USER/Kronos/backend_kronos"

cd $EXPECTED_DIR

# Start by building the database container.
./scripts/init_db.sh

# Then, build the backend container, in a new shell.
wt.exe --window last new-tab --title "Backend-Actix" wsl -e bash -i -c "../scripts/start_back.sh"

# Then, build the frontend, (non-containerized,) in a new shell. 
wt.exe --window last new-tab --title "Frontend-Vite" wsl -e bash -i -c "../scripts/start_front.sh"