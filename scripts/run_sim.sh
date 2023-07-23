#!/bin/bash

. ./scripts/helper/setup_host.sh

# curl -d "$*" -X POST $HOST/run --http1.1
docker run --network host --rm -it curlimages/curl:7.80.0 -s -d "$*" -X POST $HOST/run --http1.1
echo ""
