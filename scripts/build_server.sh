#!/bin/bash

git pull

docker build -t ghcr.io/derrick56007/codetanks_worker_server:latest -f server/Dockerfile .
docker compose up -d --force-recreate --remove-orphans server