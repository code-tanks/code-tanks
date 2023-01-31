#!/bin/bash

docker build -t ghcr.io/code-tanks/code-tanks-builder:latest -f worker_builder/Dockerfile .
docker compose up -d --force-recreate --remove-orphans builder
