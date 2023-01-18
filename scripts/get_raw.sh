#!/bin/bash

export $(xargs < ./scripts/.env)

curl $HOST/raw/$1 --http1.1
echo ""
