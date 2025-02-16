#!/bin/bash

echo "Executing basic tests"

echo "test1: healthcheck"
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8000/health_check)

if [ "$response" == "200" ]; then
    echo "Success: Received HTTP/1.1 200 OK"
else
    echo "Failure: Received HTTP/$response"
    exit 1
fi

echo ""