#!/bin/bash

# External integration tests for the service

source ./curl_expect.sh

echo " Block C: Service Tests"

# Test 1: Ensure that a blank api request fails
test_service_response "Test 1: Blank API call fails." "POST" "http://127.0.0.1:8000/api" "400"
