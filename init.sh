#!/bin/bash
redis-server --daemonize yes
redis-cli ping

/ocypod/ocypod/target/release/ocypod-server ocypod.toml &

pg_ctlcluster 12 main start

sudo -u postgres psql -c "ALTER USER postgres WITH PASSWORD 'postgres';"

# sleep 10

# export DB_URL=postgres://postgres:postgres@localhost:5432
# export OCYPOD_URL=localhost:8023

cargo run --bin ctserver --profile dev & cargo run --bin ctsim --profile dev & cargo run --bin ctbuilder --profile dev