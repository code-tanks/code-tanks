#!/bin/bash

. ./scripts/helper/setup_host.sh

# ./scripts/dev/reset_db.sh

OUTPUT=$(./scripts/upload_tank.sh examples/dart/spin_bot.dart)


if [[ $OUTPUT != "83084a8" ]]
then
  echo "Failed to upload examples/dart/spin_bot.dart"
  echo "Expected: 83084a8"
  echo "Got: $OUTPUT"
  exit 1
fi

sleep 30

OUTPUT=$(./scripts/get_build_log.sh "83084a8")


# this code needs fixing
if [[ "${OUTPUT}" == '"404"' ]]
then
  echo "Failed to get build log for 83084a8"
  echo 'Got: "404"'
  exit 1
fi

echo "build log:"
echo "${OUTPUT}"

# docker compose logs builder

OUTPUT=$(./scripts/get_raw.sh "83084a8")
RAW="$(<examples/dart/spin_bot.dart)"

if [[ "${OUTPUT}" != "${RAW}" ]]
then
  echo "Failed to get raw for 83084a8"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  exit 1
fi

RAW="$(cat ./scripts/test/expected/test_spin_bot.txt)"

OUTPUT=$(./scripts/run_sim.sh "83084a8" "83084a8")
if [[ "${OUTPUT}" != "waiting to build" ]]
then
  echo "Failed run sim for 83084a8"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "waiting to build"
  exit 1
fi

sleep 120
OUTPUT=$(./scripts/run_sim.sh "83084a8" "83084a8")

echo "${OUTPUT}" > ./scripts/test/output.txt
tr -d '\r'  < ./scripts/test/output.txt > ./scripts/test/output1.txt

echo "${RAW}" > ./scripts/test/raw.txt

if cmp ./scripts/test/output1.txt ./scripts/test/raw.txt;
then
    echo "success"
else
  echo "Failed determinism for sim 83084a8"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  docker compose logs simulator
  # docker compose logs server
#   docker ps
  # docker compose logs
  exit 1
fi

echo "sim:"
echo "${OUTPUT}"