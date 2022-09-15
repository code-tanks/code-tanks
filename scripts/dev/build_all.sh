#!/bin/bash

git pull

./scripts/dev/build_builder.sh
./scripts/dev/build_server.sh
./scripts/dev/build_simulator.sh
./scripts/dev/build_web.sh