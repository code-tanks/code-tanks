#!/bin/bash
redis-server --daemonize yes
redis-cli ping

ocypod-server ocypod.toml &

pg_ctlcluster 12 main start

sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';"


ctserver & ctsim & ctbuilder