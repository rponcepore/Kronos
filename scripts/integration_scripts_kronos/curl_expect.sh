#!/bin/bash

# This file defines a helper function for testing. Eventually I'll refactor the other tests to mimic it.

test_service_response() {
    local testname=$1
    local method=$2
    local url=$3
    local expected_status=$4
    local expected_body=$5
    local datatype=$6 #-H "Content-Type:application/json"
    local body=$7

    local response_catch="response_body.txt"

    echo "  $testname"
    echo "    URL: $url"
    echo "    Expected_Code: "$expected_status""
    # echo "    Body: "$body""
    # echo "    Datatype: "$datatype" "
    touch $response_catch

    if [ -z "$datatype" ]; then
        http_code=$(curl -X "$method" -s -o "$response_catch" -w "%{response_code}" "$url")
    else
        datatype="Content-Type: application/json"  # explicit JSON
        echo "    Header: "$datatype" "
        echo "    Body: "$body" "
        http_code=$(curl -X "$method" -s -o "$response_catch" -H "$datatype" -d "$body" -w "%{response_code}" "$url")
    fi


    # Send the curl request and capture the response status code
    
    #http_code=$(tail -n1 <<< "$response") # Extract the HTTP status code
    response_body=$(cat "$response_catch") 2>/dev/null  # Extract the body.


    # Compare the response code to the expected status
    if [ "$http_code" == "$expected_status" ]; then
        if [ "$expected_body" == "" ]; then
            echo "  Success: Received HTTP/$http_code"
        else
            if  echo "$response_body" | grep -q "$expected_body" ; then
                echo "  Success: Received HTTP/1.1 $http_code OK and correct response body:"
                # echo "  Response Body: $response_body"
            else
                echo "^^FAILURE: Incorrect response body; does not contain $expected_body"
                echo "  Response Body: $response_body"
            fi
        fi
    else
        if [ "$http_code" == "000" ]; then
            echo "^^Error: Received HTTP/$http_code. Are you sure the webserver has started?"
            cat "$response_catch"
        else
            echo "^^Failure: Received HTTP/$http_code. Expected HTTP/"$expected_status.""
            echo "Server message: "$response_body""
        fi
    fi
    echo ""
rm response_body.txt
}