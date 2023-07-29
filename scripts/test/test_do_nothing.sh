#!/bin/bash

. ./scripts/helper/setup_host.sh

# ./scripts/dev/reset_db.sh

OUTPUT=$(./scripts/upload_tank.sh examples/dart/do_nothing.dart)


if [[ $OUTPUT != "faec064" ]]
then
  echo "Failed to upload examples/dart/do_nothing.dart"
  echo "Expected: faec064"
  echo "Got: $OUTPUT"
  exit 1
fi

sleep 30

OUTPUT=$(./scripts/get_build_log.sh "faec064")


# this code needs fixing
if [[ "${OUTPUT}" == '"404"' ]]
then
  echo "Failed to get build log for faec064"
  echo 'Got: "404"'
  exit 1
fi

echo "build log:"
echo "${OUTPUT}"

# docker compose logs builder

OUTPUT=$(./scripts/get_raw.sh "faec064")
RAW="$(<examples/dart/do_nothing.dart)"

if [[ "${OUTPUT}" != "${RAW}" ]]
then
  echo "Failed to get raw for faec064"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  exit 1
fi

RAW='faec064
0|10,0,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677
{"faec064-faec064-0":{"damage_given":0,"health":100,"index":0,"tank_hash":"faec064"},"tanks":["faec064"],"winner":"faec064-faec064-0","winner_index":0}'

OUTPUT=$(./scripts/run_sim.sh "faec064")
if [[ "${OUTPUT}" != "waiting to build" ]]
then
  echo "Failed run sim for faec064"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "waiting to build"
  exit 1
fi

sleep 30
OUTPUT=$(./scripts/run_sim.sh "faec064")

echo "${OUTPUT}" > ./scripts/test/output.txt
tr -d '\r'  < ./scripts/test/output.txt > ./scripts/test/output1.txt

echo "${RAW}" > ./scripts/test/raw.txt

if cmp ./scripts/test/output1.txt ./scripts/test/raw.txt;
then
    echo "success"
else
  echo "Failed determinism for sim faec064"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  docker compose logs simulator
  exit 1
fi

echo "sim:"
echo "${OUTPUT}"