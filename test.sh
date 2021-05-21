#!/bin/sh
echo "[E2E] Testing http_cache.. "
echo "[E2E] Expecting server to run on 127.0.0.1:8080.. "
key=$(date +%s)

echo "[E2E] Testing access to a non initialized key: $key"
response=$(curl --write-out '%{http_code}' --silent --output /dev/null localhost:8080/$key)
if [ "$response" == "404" ]; then 
    echo "✅ 404 as expected"
else 
    echo 🛑 $response came as a surprise!
fi

echo "[E2E] Testing adding data to a key.. "
response=$(curl --write-out '%{http_code}' --silent --output /dev/null localhost:8080/$key -d "Hello World!")
if [ "$response" == "200" ]; then 
    echo "✅ 200 as expected"
else 
    echo 🛑 $response came as a surprise!
fi

echo "[E2E] Testing retrieving the value of the new key.. "
response=$(curl --silent localhost:8080/$key)
if [ "$response" == "Hello World!" ]; then 
    echo "✅ \"Hello World!\" as expected"
else 
    echo 🛑 $response came as a surprise!
fi

sleep 2

echo "[E2E] Testing access to key after timeout"
response=$(curl --write-out '%{http_code}' --silent --output /dev/null localhost:8080/$key)
if [ "$response" == "404" ]; then 
    echo "✅ 404 as expected"
else 
    echo 🛑 $response came as a surprise!
fi
