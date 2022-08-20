#!/bin/bash

git pull

docker build -t ghcr.io/derrick56007/codetanks_worker_simulator:latest -f worker_simulator/Dockerfile .
docker compose up -d --force-recreate --remove-orphans simulator