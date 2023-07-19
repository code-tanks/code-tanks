#!/bin/bash

export $(xargs < ./scripts/.env)

# use the find command to recursively find all files from the starting directory
# then use a while-read loop to read each line (file path) from the find command
# and perform the command on it
find "$1" -type f | while read -r file; do
    # replace 'echo' with the command you want to perform on each file
    ./scripts/official/upload_tank.sh "$file"
done