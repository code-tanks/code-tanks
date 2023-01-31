#!/bin/bash

export $(xargs < ./scripts/.env)

curl $HOST/log/$1 --http1.1
echo ""
