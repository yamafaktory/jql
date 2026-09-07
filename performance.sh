#!/usr/bin/env bash

REPORT=${REPORT:-$(pwd)/PERFORMANCE.md}
PERFORMANCE_TMP_DIR=$(pwd)/performance_tmp
MD_FILES="$PERFORMANCE_TMP_DIR/"*".md"
ASSET=$(pwd)/assets/github-repositories.json
MIN_RUNS=1000
LARGE_MIN_RUNS=50
COPIES=10

# Remove export file if present.
rm -f $REPORT

# Create the directory.
mkdir $PERFORMANCE_TMP_DIR

# Build the large working sets from the asset instead of committing them.
LARGE_JSON_FILE="$PERFORMANCE_TMP_DIR/large.json"
MULTI_JSON_FILE="$PERFORMANCE_TMP_DIR/multi.json"

jq -n --slurpfile asset "$ASSET" \
    "[range($COPIES)] | map(\$asset[0]) | add" > "$LARGE_JSON_FILE"

for _ in $(seq $COPIES); do cat "$ASSET"; done > "$MULTI_JSON_FILE"

# Run the benchmarks.
hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/OBJECT.md" \
    --min-runs $MIN_RUNS \
    "echo '{ \"foo\": \"bar\" }' | jq '.foo'" \
    "echo '{ \"foo\": \"bar\" }' | jql '\"foo\"'"

hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/ARRAY_INDEX.md" \
    --min-runs $MIN_RUNS \
    "echo '[1, 2, 3]' | jq '.[0]'" \
    "echo '[1, 2, 3]' | jql '[0]'"

hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/ARRAY_FLATTEN.md" \
    --min-runs $MIN_RUNS \
    "echo '[1, [2], [[3]]]' | jq 'flatten'" \
    "echo '[1, [2], [[3]]]' | jql '..'"

hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/PROPERTY_SELECTION_LARGE_JSON.md" \
    --min-runs $LARGE_MIN_RUNS \
    "cat $LARGE_JSON_FILE | jq -r '[.[] | {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null" \
    "cat $LARGE_JSON_FILE | jql '|>{\"name\", \"url\", \"language\", \"stargazers_count\", \"watchers_count\"}' > /dev/null"

hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/PROPERTY_SELECTION_MULTIPLE_DOCUMENTS.md" \
    --min-runs $LARGE_MIN_RUNS \
    "cat $MULTI_JSON_FILE | jq -r '[.[] | {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null" \
    "cat $MULTI_JSON_FILE | jql '|>{\"name\", \"url\", \"language\", \"stargazers_count\", \"watchers_count\"}' > /dev/null"

# Merge all the markdown files into the performance one.
for md_file in $MD_FILES; do (cat "${md_file}"; echo) >> $REPORT; done

# Remove the directory.
rm -R -f $PERFORMANCE_TMP_DIR
