#!/bin/bash
docker run -d --name $1 localhost:5001/$1
# docker network rm $1$1