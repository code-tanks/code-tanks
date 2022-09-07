#!/bin/bash

git pull
docker compose up -d db ocypod ocypod-redis registry pgAdmin
