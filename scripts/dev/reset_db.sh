#!/bin/bash

git pull

docker cp ./scripts/dev/sql/reset.sql db:/
docker exec db /bin/sh -c 'psql -h localhost -d postgres -U postgres -p 5432 -a -q -f /reset.sql'

# -h PostgreSQL server IP address
# -d database name
# -U user name
# -p port which PostgreSQL server is listening on
# -f path to SQL script
# -a all echo
# -q quiet