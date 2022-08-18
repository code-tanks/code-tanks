#!/bin/bash

git pull

docker compose up -d db ocypod ocypod-redis registry pgAdmin
docker compose up -d --build --force-recreate --remove-orphans web