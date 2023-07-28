#!/bin/bash

export $(xargs < ./scripts/.env)

tank_hash=$(bash -c 'echo -n "$(cat $1)0" | sha256sum - | cut -d" " -f1' -- "$1" | cut -c 1-7)

docker build -f ./scripts/Dockerfiles/${1##*.}.Dockerfile --build-arg url=$1 -t $tank_hash .
echo $tank_hash
