#!/bin/bash

source ./curl_expect.sh

echo " Block D: Integration Tests"

# Test 3: Send a small (legitimate) KronosRequest to the API
expected_response_body=""
datatype="json"
json_data="{\"http_method\":\"POST\", \"action\":\"get_plans\", \"unit\":\"WJH8C0\"}"

test_service_response \
"Test 1: Correct API call succeeds (with database connection)" \
"POST" \
"http://127.0.0.1:8000/api" \
"200" \
"$expected_response_body" \
"$datatype" \
"$json_data" \

