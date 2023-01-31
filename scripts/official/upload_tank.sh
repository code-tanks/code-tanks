#!/bin/bash

export $(xargs < ./scripts/.env)

curl --data-binary "@$1" -H 'Content-Type: text/plain' -X POST $HOST/upload/${1##*.} --http1.1
echo ""
