#!/bin/bash

git pull

docker build -t ghcr.io/derrick56007/codetanks_worker_builder:latest -f worker_builder/Dockerfile .
docker compose up -d --force-recreate --remove-orphans builder