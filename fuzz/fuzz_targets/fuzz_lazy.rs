#![no_main]

use libfuzzer_sys::fuzz_target;
use serde::Deserialize;
use serde_json::Value;

// Splits the input as `query \n json`, runs it through both the Value-based
// runner (the oracle) and the tape-based lazy evaluator, and asserts they agree.
// Inputs that are not `query \n <single valid JSON document>` are skipped.
fuzz_target!(|data: &[u8]| {
    let Some(newline) = data.iter().position(|&byte| byte == b'\n') else {
        return;
    };
    let (query, rest) = data.split_at(newline);
    let json = &rest[1..];

    let (Ok(query), Ok(json_str)) = (std::str::from_utf8(query), std::str::from_utf8(json)) else {
        return;
    };

    let mut deserializer = serde_json::Deserializer::from_str(json_str);
    deserializer.disable_recursion_limit();
    let Ok(value) =
        serde_json::Value::deserialize(serde_stacker::Deserializer::new(&mut deserializer))
    else {
        return;
    };

    let oracle = jql_runner::runner::raw(query, &value);
    let lazy = jql_runner::lazy::raw(query, json);

    match (oracle, lazy) {
        (Ok(oracle), Ok(lazy)) => assert!(
            values_match(&oracle, &lazy),
            "output divergence\n  query: {query:?}\n  json:  {json_str:?}\n  oracle: {oracle}\n  lazy:   {lazy}"
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

/// Structural equality, but two numbers match if they are within a few ULP:
/// simd-json and serde_json can round the last bit of an exponent-only literal
/// like `123456789012345678e9` differently (fractional forms are handled by
/// `lazy`'s own scan). Object key order is still compared.
fn values_match(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            x == y
                || match (x.as_f64(), y.as_f64()) {
                    (Some(x), Some(y)) => {
                        (x - y).abs() <= 8.0 * f64::EPSILON * x.abs().max(y.abs()).max(1.0)
                    }
                    _ => false,
                }
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(x, y)| values_match(x, y))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .zip(y)
                    .all(|((kx, vx), (ky, vy))| kx == ky && values_match(vx, vy))
        }
        _ => a == b,
    }
}
