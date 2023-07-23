#!/bin/bash

. ./scripts/helper/setup_host.sh

OUTPUT=$(./scripts/upload_tank.sh examples/dart/do_nothing.dart)


if [[ $OUTPUT != "2s2wgkh" ]]
then
  echo "Failed to upload examples/dart/do_nothing.dart"
  echo "Expected: 2s2wgkh"
  echo "Got: $OUTPUT"
fi

OUTPUT=$(./scripts/get_build_log.sh "2s2wgkh")

sleep 60

if [[ "${OUTPUT}" == '"404"' ]]
then
  echo "Failed to get build log for 2s2wgkh"
  echo 'Got: "404"'
fi

OUTPUT=$(./scripts/get_raw.sh "2s2wgkh")
RAW="$(<examples/dart/do_nothing.dart)"

if [[ "${OUTPUT}" != "${RAW}" ]]
then
  echo "Failed to get raw for 2s2wgkh"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "${RAW}"
fi

RAW='2s2wgkh
0|10,0,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677,0,0,-0.70710677,0.70710677
{"2s2wgkh-2s2wgkh-0":{"damage_given":0,"health":100,"index":0,"tank_id":"2s2wgkh"},"tanks":["2s2wgkh"],"winner":"2s2wgkh-2s2wgkh-0","winner_index":0}'

OUTPUT=$(./scripts/run_sim.sh "2s2wgkh")
if [[ "${OUTPUT}" != "waiting to build" ]]
then
  echo "Failed run sim for 2s2wgkh"
  echo 'Got:'
  echo "${OUTPUT}"
  echo 'Expected:'
  echo "waiting to build"
fi

sleep 60


OUTPUT=$(./scripts/run_sim.sh "2s2wgkh")
# OUTPUT="$(echo $OUTPUT | xargs)"
# RAW="$(echo $RAW | xargs)"

echo "${OUTPUT}" > ./scripts/test/output.txt
# sed 's/^M//' ./scripts/test/output.txt > ./scripts/test/output.txt
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
fi