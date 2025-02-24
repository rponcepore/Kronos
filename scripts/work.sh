#!/bin/bash

echo "Standing up dev environment"
echo "The docker daemon must be running in order to execute this script."
echo "For Windows users, start docker desktop."
echo "Database - local at Port 6000 (local) Port ____ (docker)"
echo "Webserver - local at Port 9001 (local) Port ____ (docker)"
echo "Nginx - local at Port 9000 (local) Port 80 (docker)"

# docker compose up -d --build webserver database
