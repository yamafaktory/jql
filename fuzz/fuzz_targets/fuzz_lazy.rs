#![no_main]

use libfuzzer_sys::fuzz_target;

// Splits the input as `query \n json`, runs it through both the Value-based
// runner (the oracle) and the tape-based lazy evaluator, and asserts they agree
// exactly. Inputs that are not `query \n <single valid JSON document>` are
// skipped.
fuzz_target!(|data: &[u8]| {
    let Some(newline) = data.iter().position(|&byte| byte == b'\n') else {
        return;
    };
    let (query, rest) = data.split_at(newline);
    let json = &rest[1..];

    let (Ok(query), Ok(json_str)) = (std::str::from_utf8(query), std::str::from_utf8(json)) else {
        return;
    };

    if has_exponent_or_long_mantissa(json) {
        return;
    }

    // The default recursion limit is left in place so that serializing the
    // results below cannot overflow the stack; `parity.rs` covers the
    // deep-nesting fall back explicitly.
    let documents: Vec<serde_json::Value> = {
        let stream = serde_json::Deserializer::from_str(json_str).into_iter::<serde_json::Value>();
        let Ok(documents) = stream.collect::<Result<Vec<_>, _>>() else {
            return;
        };

        documents
    };

    if documents.is_empty() {
        return;
    }

    // Every document evaluated by the Value runner is the oracle for the whole
    // input, covering the tape-per-document path in `raw_all` as well as the
    // single-document one in `raw`.
    let oracle: Result<Vec<_>, _> = documents
        .iter()
        .map(|document| jql_runner::runner::raw(query, document))
        .collect();
    let lazy = jql_runner::lazy::raw_all(query, json);

    match (oracle, lazy) {
        (Ok(oracle), Ok(lazy)) => assert_eq!(
            serde_json::to_string(&oracle).unwrap(),
            serde_json::to_string(&lazy).unwrap(),
            "output divergence\n  query: {query:?}\n  json:  {json_str:?}"
        ),
        (Err(oracle), Err(lazy)) => assert_eq!(
            oracle, lazy,
            "error divergence\n  query: {query:?}\n  json:  {json_str:?}"
        ),
        (oracle, lazy) => panic!(
            "ok/err divergence\n  query: {query:?}\n  json:  {json_str:?}\n  oracle: {oracle:?}\n  lazy:   {lazy:?}"
        ),
    }
});

/// Whether `json` holds a digit run of 16 or more, or a digit immediately before
/// an `e`/`E`.
///
/// Those are the mantissas simd-json and serde_json can round differently.
/// `lazy` skips the tape for the long fractional ones; for scientific notation
/// it accepts a one-ULP difference, because recognising it in production would
/// mean a quote-aware scan (see `numbers_may_diverge`). Skipping such inputs
/// here — over-eagerly, which costs nothing on fuzz-sized input — keeps both
/// comparisons below exact rather than hiding real divergence behind a
/// tolerance.
fn has_exponent_or_long_mantissa(json: &[u8]) -> bool {
    let mut digits = 0;

    for &byte in json {
        if byte.is_ascii_digit() {
            digits += 1;

            if digits >= 16 {
                return true;
            }
        } else if matches!(byte, b'e' | b'E') {
            if digits > 0 {
                return true;
            }
        } else if byte != b'.' {
            digits = 0;
        }
    }

    false
}
