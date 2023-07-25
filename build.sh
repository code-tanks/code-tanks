#!/bin/bash

docker build --progress=plain -t ctall .
docker run -p 8088:8088 -e DB_URL=postgres://postgres:example@db:5432 -e OCYPOD_URL=localhost:8023 ctall