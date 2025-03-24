#!/bin/bash

# External integration tests for the service

source ./curl_expect.sh

echo " Block C: Service Tests"

# Test 1: Ensure that a blank api request fails
test_service_response "Test 1: Blank API call fails." "POST" "http://127.0.0.1:8000/api" "400"


# Test 2: Send a small (legitimate) KronosRequest to the API
expected_response_body=""
datatype="json"
json_data="{\"http_method\":\"POST\", \"action\":\"get_plans\", \"unit\":\"tstUIC\"}"

test_service_response \
"Test 2: Correct API call succeeds (no database connection)" \
"POST" \
"http://127.0.0.1:8000/api" \
"200" \
"$expected_response_body" \
"$datatype" \
"$json_data" \


# Test 4: Include null values
expected_response_body=""
datatype="json"
json_data="{\"http_method\":\"POST\", \"action\":, \"unit\":\"WJH8C0\"}" #Note bad reqeust format (no action field)

test_service_response \
"Test 3: Bad reqeust returns 400 code" \
"POST" \
"http://127.0.0.1:8000/api" \
"400" \
"$expected_response_body" \
"$datatype" \
"$json_data" \

# Test 4: Include null values
expected_response_body="Request action field is null."
datatype="json"
json_data="{\"http_method\":\"POST\", \"action\":null, \"unit\":\"WJH8C0\"}" #Note bad reqeust format (no action field)

test_service_response \
"Test 4: Correct API call succeeds (with database connection)" \
"POST" \
"http://127.0.0.1:8000/api" \
"400" \
"$expected_response_body" \
"$datatype" \
"$json_data" \