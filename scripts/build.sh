#!/bin/bash

docker compose up -d db ocypod ocypod-redis registry builder pgAdmin
docker compose up -d --build --force-recreate --remove-orphans server builder