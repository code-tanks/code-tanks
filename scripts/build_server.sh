#!/bin/bash

git pull

docker build -t ghcr.io/code-tanks/code-tanks-server:latest -f server/Dockerfile .
docker compose up -d --force-recreate --remove-orphans server