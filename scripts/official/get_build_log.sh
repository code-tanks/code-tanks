#!/bin/bash

export $(xargs < ./scripts/.env)

# curl $HOST/log/$1 --http1.1
docker run --network host --rm -it curlimages/curl:7.80.0 $HOST/log/$1 --http1.1
echo ""
