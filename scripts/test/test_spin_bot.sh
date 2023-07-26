#!/bin/bash

. ./scripts/helper/setup_host.sh

# ./scripts/dev/reset_db.sh

OUTPUT=$(./scripts/dev/no-docker/upload_tank.sh examples/dart/spin_bot.dart)


if [[ $OUTPUT != "1wexiev" ]]
then
  echo "Failed to upload examples/dart/spin_bot.dart"
  echo "Expected: 1wexiev"
  echo "Got: $OUTPUT"
  exit 1
fi

sleep 30

OUTPUT=$(./scripts/dev/no-docker/get_build_log.sh "1wexiev")


# this code needs fixing
if [[ "${OUTPUT}" == '"404"' ]]
then
  echo "Failed to get build log for 1wexiev"
  echo 'Got: "404"'
  exit 1
fi

echo "build log:"
echo "${OUTPUT}"

# docker compose logs builder

OUTPUT=$(./scripts/dev/no-docker/get_raw.sh "1wexiev")
RAW="$(<examples/dart/spin_bot.dart)"

if [[ "${OUTPUT}" != "${RAW}" ]]
then
  echo "Failed to get raw for 1wexiev"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  exit 1
fi

RAW='1wexiev
0|10,0,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677
{"1wexiev-1wexiev-0":{"damage_given":0,"health":100,"index":0,"tank_id":"1wexiev"},"tanks":["1wexiev"],"winner":"1wexiev-1wexiev-0","winner_index":0}'

OUTPUT=$(./scripts/dev/no-docker/run_sim.sh "1wexiev" "1wexiev")
if [[ "${OUTPUT}" != "waiting to build" ]]
then
  echo "Failed run sim for 1wexiev"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "waiting to build"
  exit 1
fi

sleep 60
OUTPUT=$(./scripts/dev/no-docker/run_sim.sh "1wexiev" "1wexiev")

echo "${OUTPUT}" > ./scripts/test/output.txt
tr -d '\r'  < ./scripts/test/output.txt > ./scripts/test/output1.txt

echo "${RAW}" > ./scripts/test/raw.txt

if cmp ./scripts/test/output1.txt ./scripts/test/raw.txt;
then
    echo "success"
else
  echo "Failed determinism for sim 1wexiev"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  # docker compose logs simulator
  # docker compose logs server
#   docker ps
  # docker compose logs
  exit 1
fi

echo "sim:"
echo "${OUTPUT}"