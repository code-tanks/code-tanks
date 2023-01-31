#!/bin/bash

export $(xargs < ./scripts/.env)

curl $HOST/sim_log/$1 --http1.1
echo ""
