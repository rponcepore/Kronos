#!/bin/bash

cd /home/$USER/Kronos/scripts/integration_scripts_kronos/

echo "Running all tests"

./basic_tests.sh

./database_tests.sh

./service_tests.sh

# Done
echo "Tests complete."