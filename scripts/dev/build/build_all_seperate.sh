#!/bin/bash

sh -c './scripts/dev/build/build_builder.sh & ./scripts/dev/build/build_server.sh & ./scripts/dev/build/build_simulator.sh & ./scripts/dev/build/build_web.sh & wait'
