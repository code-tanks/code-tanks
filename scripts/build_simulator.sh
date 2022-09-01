#!/bin/bash

git pull

docker build -t ghcr.io/derrick56007/code-tanks-simulator:latest -f worker_simulator/Dockerfile .
docker compose up -d --force-recreate --remove-orphans simulator