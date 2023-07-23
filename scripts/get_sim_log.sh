#!/bin/bash

. ./scripts/helper/setup_host.sh

# curl $HOST/sim_log/$1 --http1.1
docker run --network host --rm -it curlimages/curl:7.80.0 -s $HOST/sim_log/$1 --http1.1
echo ""
