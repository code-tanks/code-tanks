#!/bin/bash

for i in examples/*/**; do
    echo "Testing example: $i"

    out=$(./scripts/build.sh $i);
    docker rm $out --force
    docker run -d -p "8080:8080" --name $out $out
    sleep 5
    request_commands=$(curl -sS localhost:8080/request_commands)
    echo "request_commands: $request_commands"

    if echo "$request_commands" | jq -e '. | type == "array"' > /dev/null; then
        echo "Valid JSON list from /request_commands"
    else
        echo "Not a valid JSON list from /request_commands"
        exit 1
    fi

    request_commands_by_event=$(curl -sS -X POST -H "Content-Type: application/json" -d '{"key": "value"}' localhost:8080/request_commands_by_event)

    echo "request_commands_by_event: $request_commands_by_event"

    if echo "$request_commands_by_event" | jq -e '. | type == "array"' > /dev/null; then
        echo "Valid JSON list from /request_commands_by_event"
    else
        echo "Not a valid JSON list from /request_commands_by_event"
        exit 1
    fi

    docker rm $out --force
done