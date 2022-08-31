#!/bin/bash

curl --data-binary "@$1" -H 'Content-Type: text/plain' -X POST http://localhost:8089/upload/${1##*.}
echo ""
