#!/bin/bash

export $(xargs < ./scripts/.env)

# curl $HOST/raw/$1 --http1.1
docker run --network host --rm curlimages/curl:7.80.0 $HOST/raw/$1 --http1.1
echo ""
