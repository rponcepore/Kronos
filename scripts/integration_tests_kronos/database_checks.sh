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
fi
echo ""
# TEST 4: DATABASE_HEALTHCHECK