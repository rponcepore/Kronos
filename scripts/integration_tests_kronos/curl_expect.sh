#!/bin/bash

# This file defines a helper function for testing. Eventually I'll refactor the other tests to mimic it.

test_service_response() {
    local testname=$1
    local method=$2
    local url=$3
    local expected_status=$4
    local expected_body=$5

    echo "  $testname"
    echo "    URL: $url"
    echo "    Expected_Code: $expected_status"
    
    touch response_body.txt

    # Send the curl request and capture the response status code
    response=$(curl -X $method -s -o response_body.txt -w "%{http_code}" "$url")
    http_code=$(tail -n1 <<< "$response") # Extract the HTTP status code

    response_body=$(cat response_body.txt) 2>/dev/null  # Extract the body.


    # Compare the response code to the expected status
    if [ "$http_code" == "$expected_status" ]; then
        if [ "$expected_body" == "" ]; then
            echo "  Success: Received HTTP/$response"
            rm response_body.txt
        else
            if  echo "$response_body" | grep -q "$expected_body" ; then
                echo "  Success: Received HTTP/1.1 $http_code OK and correct response body:"
                echo "  Response Body: $response_body"
                rm response_body.txt
            else
                echo "^^FAILURE: Received HTTP/1.1 200 OK but the response body does not contain 'health_check_body called!'"
                echo "  Response Body: $response_body"
                rm response_body.txt
            fi
        fi
    else
        if [ "$http_code" == "000" ]; then
            echo "^^Error: Received HTTP/$http_code. Are you sure the webserver has started?"
        else
            echo "^^Failure: Received HTTP/$http_code. Expected HTTP/$expected_status."
        fi
    fi
    echo ""
}