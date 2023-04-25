#!/usr/bin/env bash

LARGE_JSON_FILE=$(pwd)/assets/github-repositories.json
MIN_RUNS=10

hyperfine \
    --min-runs $MIN_RUNS \
    "cat $LARGE_JSON_FILE | jq -r '[.[] | {name: .name, full_name: .full_name}]' > /dev/null" \
    "cat $LARGE_JSON_FILE | jql '|>{\"name\", \"full_name\"}' > /dev/null"

hyperfine \
    --min-runs $MIN_RUNS \
    "cat $LARGE_JSON_FILE | jq -r '[.[] | {name: .name, full_name: .full_name}]' > /dev/null" \
    "cat $LARGE_JSON_FILE | ./target/release/jql '|>{\"name\", \"full_name\"}' > /dev/null"
