#!/bin/bash

echo "Executing basic tests"

# TEST 1: HEALTHCHECK
echo "test1: HEALTHCHECK"
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8000/health_check)

if [ "$response" == "200" ]; then
    echo "Success: Received HTTP/1.1 200 OK"
else
    if [ "$response" == "000" ]; then
        echo "Error: Received HTTP/$response. Are you sure that the webserver has started?"
    else
        echo "Failure: Received HTTP/$response"
    fi  
exit 1
fi
echo ""
# END TEST 1: HEALTHCHECK

# TEST 2: PHONY HEALTHCHECK
echo "test2: PHONY HEALTHCHECK"
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8000/health_check_phony)

if [ "$response" != "200" ]; then
    echo "Success: Received HTTP/1.1 404, which is correct."
else
    if [ "$response" == "000" ]; then
        echo "Error: Received HTTP/$response. Are you sure that the webserver has started?"
        exit 1
    else
        echo "Success: Received HTTP/$response"
        Exit 0
    fi  
fi
echo ""
# END TEST 2: PHONY HEALTHCHECK