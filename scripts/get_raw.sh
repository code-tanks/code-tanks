#!/bin/bash

. ./scripts/helper/setup_host.sh

# curl $HOST/raw/$1 --http1.1
docker run --network host --rm curlimages/curl:7.80.0 -s $HOST/raw/$1 --http1.1
echo ""
