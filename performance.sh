
PERFORMANCE_DIR=$(pwd)/performance

echo $PERFORMANCE_DIR

rm -R -f $PERFORMANCE_DIR

hyperfine \
    --export-markdown $PERFORMANCE_DIR/OBJECT.md \
    --min-runs 1 \
    "echo '{ \"foo\": \"bar\" }' | jq '.foo'" \
    "echo '{ \"foo\": \"bar\" }' | jql '."foo"'"

hyperfine \
    --export-markdown $PERFORMANCE_DIR/ARRAY_INDEX.md \
    --min-runs 1 \
    "echo '[1, 2, 3]' | jq '.[0]'" \
    "echo '[1, 2, 3]' | jql '.[0]'"

hyperfine \
    --export-markdown $PERFORMANCE_DIR/ARRAY_FLATTEN.md \
    --min-runs 1 \
    "echo '[1, [2], [[3]]]' | jq 'flatten'" \
    "echo '[1, [2], [[3]]]' | jql '...'"

#echo | cat ARRAY.md - FLATTEN.md > PERFORMANCE.md
echo | cat $PERFORMANCE_DIR/*.md > PERFORMANCE.md

#rm ARRAY.md FLATTEN.md