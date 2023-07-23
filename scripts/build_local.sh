#!/bin/bash

export $(xargs < ./scripts/.env)

docker build -f ./scripts/Dockerfiles/${1##*.}.Dockerfile --build-arg url=$1 -t ${1##*/} .
