#!/bin/bash

curl http://localhost:8089/raw/$1 --http1.1
echo ""
