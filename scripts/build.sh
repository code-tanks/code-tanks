#!/bin/bash

docker compose up -d db ocypod ocypod-redis pgAdmin builder
docker compose up -d --build --force-recreate --remove-orphans server builder