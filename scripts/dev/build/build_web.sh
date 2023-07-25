#!/bin/bash

docker build --progress=plain -t ghcr.io/code-tanks/code-tanks-web:latest -f web/Dockerfile .
docker compose up -d --force-recreate --remove-orphans web
