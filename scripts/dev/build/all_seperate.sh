#!/bin/bash

sh -c './scripts/dev/build/worker_builder.sh & ./scripts/dev/build/server.sh & ./scripts/dev/build/worker_simulator.sh & ./scripts/dev/build/web.sh & wait'
