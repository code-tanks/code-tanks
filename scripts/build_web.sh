#!/bin/bash

git pull

docker build -t ghcr.io/derrick56007/codetanks_web:latest -f web/Dockerfile .
docker compose up -d --force-recreate --remove-orphans web