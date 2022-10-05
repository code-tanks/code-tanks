#!/bin/bash

git pull

export DOCKER_BUILDKIT=1
docker build -t ghcr.io/code-tanks/code-tanks-simulator:latest -f worker_simulator/worker_simulator.Dockerfile .
docker compose up -d --force-recreate --remove-orphans simulator