#!/usr/bin/env bash

LARGE_JSON_FILE=$(pwd)/assets/github-repositories.json
RELEASE=$(pwd)/target/release/jql
MIN_RUNS=100

hyperfine \
    --min-runs $MIN_RUNS \
    "cat $LARGE_JSON_FILE | jql '|>{\"name\",\"full_name\"}' > /dev/null" \
    "cat $LARGE_JSON_FILE | $RELEASE '|>{\"name\",\"full_name\"}' > /dev/null"
