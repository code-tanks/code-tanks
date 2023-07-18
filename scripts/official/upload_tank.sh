#!/bin/bash

export $(xargs < ./scripts/.env)

# curl --data-binary "@$1" -H 'Content-Type: text/plain' -X POST $HOST/upload/${1##*.} --http1.1
docker run --network host --rm -it -v "$(dirname $(readlink -f $1)):/work" curlimages/curl:7.80.0 -v --data-binary @/work/${1##*/} -H 'Content-Type: text/plain' $HOST/upload/${1##*.}
echo ""
