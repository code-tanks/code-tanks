#!/bin/sh

IFS='/'
src="$*"
curl http://localhost:8089/run/$src
echo ""
