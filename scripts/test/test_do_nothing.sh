#!/bin/bash

. ./scripts/helper/setup_host.sh

# ./scripts/dev/reset_db.sh

OUTPUT=$(./scripts/dev/no-docker/upload_tank.sh examples/dart/do_nothing.dart)


if [[ $OUTPUT != "2s2wgkh" ]]
then
  echo "Failed to upload examples/dart/do_nothing.dart"
  echo "Expected: 2s2wgkh"
  echo "Got: $OUTPUT"
  exit 1
fi

sleep 30

OUTPUT=$(./scripts/dev/no-docker/get_build_log.sh "2s2wgkh")


# this code needs fixing
if [[ "${OUTPUT}" == '"404"' ]]
then
  echo "Failed to get build log for 2s2wgkh"
  echo 'Got: "404"'
  exit 1
fi

echo "build log:"
echo "${OUTPUT}"

# docker compose logs builder

OUTPUT=$(./scripts/dev/no-docker/get_raw.sh "2s2wgkh")
RAW="$(<examples/dart/do_nothing.dart)"

if [[ "${OUTPUT}" != "${RAW}" ]]
then
  echo "Failed to get raw for 2s2wgkh"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  exit 1
fi

RAW='2s2wgkh
0|10,0,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677
{"2s2wgkh-2s2wgkh-0":{"damage_given":0,"health":100,"index":0,"tank_id":"2s2wgkh"},"tanks":["2s2wgkh"],"winner":"2s2wgkh-2s2wgkh-0","winner_index":0}'

OUTPUT=$(./scripts/dev/no-docker/run_sim.sh "2s2wgkh")
if [[ "${OUTPUT}" != "waiting to build" ]]
then
  echo "Failed run sim for 2s2wgkh"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "waiting to build"
  exit 1
fi

sleep 30
OUTPUT=$(./scripts/dev/no-docker/run_sim.sh "2s2wgkh")

echo "${OUTPUT}" > ./scripts/test/output.txt
tr -d '\r'  < ./scripts/test/output.txt > ./scripts/test/output1.txt

echo "${RAW}" > ./scripts/test/raw.txt

if cmp ./scripts/test/output1.txt ./scripts/test/raw.txt;
then
    echo "success"
else
  echo "Failed determinism for sim 2s2wgkh"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
  docker compose logs simulator
  exit 1
fi

echo "sim:"
echo "${OUTPUT}"