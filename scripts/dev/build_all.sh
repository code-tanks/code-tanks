#!/bin/bash

git pull

./scripts/build_builder.sh
./scripts/build_server.sh
./scripts/build_simulator.sh
./scripts/build_web.sh