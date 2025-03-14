#!/bin/bash

cd /home/$USER/Kronos/scripts/integration_tests_kronos/

./basic_tests.sh

./database_tests.sh

# Done
echo "Tests complete."