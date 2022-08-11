#!/bin/bash

docker compose up -d db redis pgAdmin
docker compose up -d --build --force-recreate --remove-orphans server