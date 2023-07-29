#!/bin/bash

. ./scripts/helper/setup_host.sh

curl -sS -d "$*" -X POST $HOST/run --http1.1
# docker run --network host --rm curlimages/curl:7.80.0 -sS -d "$*" -X POST $HOST/run --http1.1
echo ""
