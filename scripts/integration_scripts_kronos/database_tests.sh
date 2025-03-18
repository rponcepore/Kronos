#!/bin/bash

# import testing function
source ./curl_expect.sh

echo " Block B: Database Tests"

# TEST 4: DATABASE_HEALTHCHECK: Only owrks if the database is alive and connected to the backend. 
test_service_response "Test 1: Database Alive Healthcheck" "GET" "http://127.0.0.1:8000/database_health_check" "200"