#!/bin/bash

#!/bin/bash

# This file defines a helper function for testing. Eventually I'll refactor the other tests to mimic it.

source ./curl_expect.sh

echo "Printing output for a Plans Post Request"

# Test 2: Send a small (legitimate) KronosRequest to the API
expected_response_body=""
datatype="json"
json_data="{\"http_method\":\"POST\", \"action\":\"get_plans\", \"unit\":\"WJH8C0\"}"

test_service_response \
"Test 2: Correct API call succeeds." \
"POST" \
"http://127.0.0.1:8000/api" \
"200" \
"$expected_response_body" \
"$datatype" \
"$json_data" \
"true"