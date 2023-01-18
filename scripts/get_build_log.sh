#!/bin/bash

curl http://localhost:8089/log/$1 --http1.1
echo ""
