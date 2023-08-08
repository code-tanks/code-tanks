#!/bin/bash

tank_hash=$(bash -c 'sha256sum $1 | cut -d" " -f1' -- "$1" | cut -c 1-7)

docker build -f ./scripts/Dockerfiles/${1##*.}.Dockerfile --build-arg url=$(basename "$1") -t $tank_hash $(dirname "$1")
echo $tank_hash
