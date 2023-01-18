#!/bin/bash

export $(xargs < ./scripts/.env)

curl -d "$*" -X POST $HOST/run --http1.1
echo ""
