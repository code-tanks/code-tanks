#!/bin/bash

export $(xargs < ./scripts/.env)

# curl -d "$*" -X POST $HOST/run --http1.1
docker run --network host --rm -it curlimages/curl:7.80.0 -d "$*" -X POST $HOST/run --http1.1
echo ""
