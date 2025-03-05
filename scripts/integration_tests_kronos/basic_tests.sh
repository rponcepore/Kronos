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

# TEST 3: HEALTHCHECK_BODY
echo "test3: HEALTHCHECK_BODY"
response=$(curl -s -w "%{http_code}" -o response_body.txt http://127.0.0.1:8000/health_check_body)

http_code=$(tail -n1 <<< "$response")  # Extract the HTTP status code
response_body=$(cat response_body.txt)

echo "Response body: $response_body "

if [ "$http_code" == "200" ]; then
    if echo "$response_body" | grep -q "health_check_body success"; then
        echo "Success: Received HTTP/1.1 200 OK and correct response body"
        rm response_body.txt
    else
        echo "Failure: Received HTTP/1.1 200 OK but the response body does not contain 'health_check_body called!'"
        echo "Response Body: $response_body"
    fi
else
    if [ "$http_code" == "000" ]; then
        echo "Error: Received HTTP/$http_code. Are you sure that the webserver has started?"
    else
        echo "Failure: Received HTTP/$http_code"
    fi  
    echo "Response Body: $response_body"
fi

echo ""
# END TEST 3: HEALTHCHECK_BODY


# TEST 4: DATABASE_HEALTHCHECK
echo "test4: DATABASE_HEALTHCHECK"
response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8000/database_health_check)

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
# TEST 4: DATABASE_HEALTHCHECK