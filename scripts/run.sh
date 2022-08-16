#!/bin/bash
old="$IFS"
IFS='/'
str="'$*'"
curl http://localhost:8089/run/$str
echo ""
