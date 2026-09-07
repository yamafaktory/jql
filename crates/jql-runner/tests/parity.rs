//! Parity harness: every case must produce byte-identical output from the
//! `Value`-based [`jql_runner::runner`] (the oracle) and the tape-based
//! [`jql_runner::lazy`] evaluator.

use jql_runner::{
    lazy,
    runner,
};
use serde::Deserialize;
use serde_json::Value;

/// Deserializes the test input the same way the binary does, so deeply nested
/// fixtures build an oracle `Value` instead of hitting serde_json's default
/// recursion limit.
fn deserialize(json: &str) -> Value {
    let mut deserializer = serde_json::Deserializer::from_str(json);

    deserializer.disable_recursion_limit();

    Value::deserialize(serde_stacker::Deserializer::new(&mut deserializer))
        .expect("test input must be valid JSON")
}

/// Runs `query` through both evaluators and asserts they agree.
#[track_caller]
fn assert_parity(query: &str, json: &str) {
    let value = deserialize(json);

    let oracle = runner::raw(query, &value);
    let lazy = lazy::raw(query, &mut json.to_string().into_bytes());

    match (oracle, lazy) {
        (Ok(oracle), Ok(lazy)) => assert_eq!(
            serde_json::to_string(&oracle).unwrap(),
            serde_json::to_string(&lazy).unwrap(),
            "output mismatch for query `{query}` on `{json}`"
        ),
        (Err(oracle), Err(lazy)) => assert_eq!(
            oracle, lazy,
            "error mismatch for query `{query}` on `{json}`"
        ),
        (oracle, lazy) => panic!(
            "ok/err mismatch for query `{query}` on `{json}`:\n  oracle: {oracle:?}\n  lazy:   {lazy:?}"
        ),
    }
}

#[test]
fn drill_down() {
    assert_parity(r#""a""#, r#"{ "a": 1 }"#);
    assert_parity(r#""a""b""#, r#"{ "a": { "b": 2 } }"#);
    assert_parity(r#""a""b""c""#, r#"{ "a": { "b": { "c": 3 } } }"#);
    assert_parity("[0]", r#"["a", "b", "c"]"#);
    assert_parity("[2]", r#"["a", "b", "c"]"#);
    assert_parity(r#""a"[1]"#, r#"{ "a": [10, 20, 30] }"#);
    assert_parity(r#""a"[1]"b""#, r#"{ "a": [{ "b": 1 }, { "b": 2 }] }"#);
    assert_parity(r#""a""#, r#"{ "a": { "z": 1, "m": 2, "a": 3 } }"#);
    assert_parity(r#""deep""#, &format!(r#"{{ "deep": {} }}"#, nested(200)));
}

#[test]
fn drill_down_errors() {
    assert_parity(r#""missing""#, r#"{ "a": 1 }"#);
    assert_parity(r#""a""missing""#, r#"{ "a": { "b": 1 } }"#);
    assert_parity(r#""a""#, r#"[1, 2, 3]"#);
    assert_parity("[5]", r#"["a"]"#);
    assert_parity("[0]", r#"{ "a": 1 }"#);
    assert_parity(r#""a"[0]"#, r#"{ "a": 1 }"#);
}

#[test]
fn scalars_and_numbers() {
    assert_parity(r#""n""#, r#"{ "n": 0 }"#);
    assert_parity(r#""n""#, r#"{ "n": -1 }"#);
    assert_parity(r#""n""#, r#"{ "n": 18446744073709551615 }"#);
    assert_parity(r#""n""#, r#"{ "n": -9223372036854775808 }"#);
    assert_parity(r#""n""#, r#"{ "n": 3.14 }"#);
    assert_parity(r#""n""#, r#"{ "n": 1e10 }"#);
    assert_parity(r#""n""#, r#"{ "n": -0.0 }"#);
    assert_parity(r#""n""#, r#"{ "n": 1234567890123456789 }"#);
    assert_parity(
        r#""s""#,
        r#"{ "s": "with \"quotes\" and \n newline and é" }"#,
    );
    assert_parity(r#""b""#, r#"{ "b": true }"#);
    assert_parity(r#""z""#, r#"{ "z": null }"#);
    assert_parity(r#""e""#, r#"{ "e": {} }"#);
    assert_parity(r#""e""#, r#"{ "e": [] }"#);
}

#[test]
fn key_order_preserved() {
    assert_parity(r#""o""#, r#"{ "o": { "z": 1, "y": 2, "a": 3, "m": 4 } }"#);
    assert_parity(
        r#""o""nested""#,
        r#"{ "o": { "nested": { "d": 1, "c": 2, "b": 3, "a": 4 } } }"#,
    );
}

#[test]
fn delegated_non_drill_down() {
    // These fall through to the Value-based runner in `lazy`; parity is trivially
    // guaranteed but they exercise the delegation path end to end.
    assert_parity(r#""a","b""#, r#"{ "a": 1, "b": 2 }"#);
    assert_parity("[2,0]", r#"["a", "b", "c", "d"]"#);
    assert_parity("[2:0]", r#"["a", "b", "c"]"#);
    assert_parity(r#"{"c","a"}"#, r#"{ "a": 1, "b": 2, "c": 3 }"#);
    assert_parity("..", r#"[1, [2], [[3]]]"#);
    assert_parity(r#""a"@"#, r#"{ "a": { "x": 1, "y": 2 } }"#);
    assert_parity(r#""a"!"#, r#"{ "a": [1, 2, 3] }"#);
    assert_parity(
        r#""a"|>"b""c""#,
        r#"{ "a": [{ "b": { "c": 1 } }, { "b": { "c": 2 } }] }"#,
    );
    assert_parity(r#"|={"a""b""c"=2}"#, r#"[{ "a": { "b": { "c": 2 } } }]"#);
}

#[test]
fn deep_nesting_forces_fallback() {
    // Beyond simd-json's default max depth (1024): `lazy` must fall back to the
    // stacker-backed deserializer and still match the oracle. Kept modest so the
    // recursive serde_json clone/serialize stays within a test thread's stack.
    let json = format!(r#"{{ "deep": {} }}"#, nested(1100));
    assert_parity(r#""deep""#, &json);
}

#[test]
fn empty_and_parse_errors() {
    assert_parity("", "{}");
    assert_parity(r#""a"b"#, r#"{ "a": 1 }"#);
}

/// Builds `[[…[value]…]]` nested `depth` levels deep.
fn nested(depth: usize) -> String {
    format!("{}0{}", "[".repeat(depth), "]".repeat(depth))
}
