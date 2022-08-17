#!/bin/bash
docker run -d --network=simulator localhost:5001/$1
# docker network rm $1$1