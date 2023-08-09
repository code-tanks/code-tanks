#!/bin/bash

skip=("time_out.dart" "crash.py")

for i in examples/*/**; do
    found=false
    for s in "${skip[@]}"; do
        e=$(basename $i)
        if [ "$s" == "$e" ]; then
            found=true
            break
        fi
    done

    if [ "$found" = true ]; then
        echo "skipping $i"
        echo ""
        continue
    fi

    echo "Testing example: $i"

    out=$(./scripts/build.sh $i);
    docker rm $out --force
    docker run -d -p "8080:8080" --name $out $out
    sleep 5

    for j in {1..3}; do
        request_commands=$(curl -sS -m 3 localhost:8080/request_commands)
        echo "request_commands: $request_commands"
        # Trim leading and trailing spaces using parameter expansion
        request_commands="${request_commands#"${request_commands%%[![:space:]]*}"}"
        request_commands="${request_commands%"${request_commands##*[![:space:]]}"}"

        if [ -n "$request_commands" ] && echo "$request_commands" | jq -e '. | type == "array" and length > 0' > /dev/null; then
            echo "Valid JSON list from /request_commands"
        else
            echo "Not a valid JSON list from /request_commands"
            docker logs $out
            exit 1
        fi

        request_commands_by_event=$(curl -sS -m 3 -X POST -H "Content-Type: application/json" -d '{"key": "value"}' localhost:8080/request_commands_by_event)
        echo "request_commands_by_event: $request_commands_by_event"
        # Trim leading and trailing spaces using parameter expansion
        request_commands_by_event="${request_commands_by_event#"${request_commands_by_event%%[![:space:]]*}"}"
        request_commands_by_event="${request_commands_by_event%"${request_commands_by_event##*[![:space:]]}"}"

        if [ -n "$request_commands_by_event" ] && echo "$request_commands_by_event" | jq -e '. | type == "array"' > /dev/null; then
            echo "Valid JSON list from /request_commands_by_event"
        else
            echo "Not a valid JSON list from /request_commands_by_event"
            docker logs $out
            exit 1
        fi
    done

    docker rm $out --force
    echo ""
done