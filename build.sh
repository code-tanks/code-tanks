#!/bin/bash

docker build --progress=plain -t ctall .
docker run -d --network=host -e REGISTRY_HTTP_ADDR=0.0.0.0:5001 registry
docker run -p 8089:8088 -v /var/run/docker.sock:/var/run/docker.sock -e DB_URL=postgres://postgres:postgres@localhost:5432 -e OCYPOD_URL=localhost:8023 ctall