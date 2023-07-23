#!/bin/bash

# export $(xargs < ./scripts/.env)

# # curl --data-binary "@$1" -H 'Content-Type: text/plain' -X POST $HOST/upload/${1##*.} --http1.1
# docker run --network host --rm -v "$(dirname $(readlink -f $1)):/work" curlimages/curl:7.80.0 -s --data-binary @/work/${1##*/} -H 'Content-Type: text/plain' $HOST/upload/${1##*.} --http1.1
# echo ""

export $(xargs < ./scripts/.env)
 
if [ "$1" = -p ]; then
    export HOST=$OFFICIAL_HOST
    shift
fi

# echo $HOST
# echo $1