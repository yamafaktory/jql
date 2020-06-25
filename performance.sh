
REPORT=$(pwd)/PERFORMANCE.md
PERFORMANCE_TMP_DIR=$(pwd)/performance_tmp
MD_FILES="$PERFORMANCE_TMP_DIR/"*".md"
MIN_RUNS=1000

# Remove export file if present.
rm -f $REPORT

# Create the directory.
mkdir $PERFORMANCE_TMP_DIR

# Run the benchmarks.
hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/OBJECT.md" \
    --min-runs $MIN_RUNS \
    "echo '{ \"foo\": \"bar\" }' | jq '.foo'" \
    "echo '{ \"foo\": \"bar\" }' | jql '."foo"'"

hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/ARRAY_INDEX.md" \
    --min-runs $MIN_RUNS \
    "echo '[1, 2, 3]' | jq '.[0]'" \
    "echo '[1, 2, 3]' | jql '.[0]'"

hyperfine \
    --export-markdown "$PERFORMANCE_TMP_DIR/ARRAY_FLATTEN.md" \
    --min-runs $MIN_RUNS \
    "echo '[1, [2], [[3]]]' | jq 'flatten'" \
    "echo '[1, [2], [[3]]]' | jql '...'"

# Merge all the markdown files into the performance one.
for md_file in $MD_FILES; do (cat "${md_file}"; echo) >> $REPORT; done

# Remove the directory.
rm -R -f $PERFORMANCE_TMP_DIR
