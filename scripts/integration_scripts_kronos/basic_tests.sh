#!/bin/bash

# Include our bash testing function.
source ./curl_expect.sh

echo " Block A: Basic Tests"

# Test 1: Ensure that a blank api request fails
test_service_response "Test 1: Healthcheck" "GET" "http://127.0.0.1:8000/health_check" "200"

# TEST 2: PHONY HEALTHCHECK
test_service_response "Test 2: Phony Healthcheck" "GET" "http://127.0.0.1:8000/health_check_phony" "404"

# TEST 3: HEALTHCHECK_BODY
expected_response_body="health_check_body success"
test_service_response "Test 3: Body Healthcheck" "GET" "http://127.0.0.1:8000/health_check_body" "200" "$expected_response_body"




