#!/bin/sh

IFS='/'
curl http://localhost:8089/run/"$*"
echo ""
