#!/bin/bash

# External integration tests for the service

source ./curl_expect.sh

echo " Block C: Service Tests"

# Test 1: Ensure that a blank api request fails
test_service_response "Test 1: Blank API call fails." "POST" "http://127.0.0.1:8000/api" "400"

# Test 2: Send a small (legitimate) KronosRequest to the API
expected_response_body=""
datatype="json"
json_data="{\"http_method\":\"POST\", \"action\":\"get_plans\", \"unit\":\"WHJ8C0\"}"

test_service_response \
"Test 2: Correct API call succeeds." \
"POST" \
"http://127.0.0.1:8000/api" \
"200" \
"$expected_response_body" \
"$datatype" \
"$json_data" \