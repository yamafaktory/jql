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
    let lazy = lazy::raw(query, &mut json.as_bytes().to_vec());

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

const OBJECT: &str = r#"{ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 }"#;
const ARRAY: &str = r#"["a", "b", "c", "d", "e"]"#;

#[test]
fn drill_down() {
    assert_parity(r#""a""#, r#"{ "a": 1 }"#);
    assert_parity(r#""a""b""c""#, r#"{ "a": { "b": { "c": 3 } } }"#);
    assert_parity("[0]", ARRAY);
    assert_parity("[4]", ARRAY);
    assert_parity(r#""a"[1]"b""#, r#"{ "a": [{ "b": 1 }, { "b": 2 }] }"#);
    assert_parity(r#""deep""#, &format!(r#"{{ "deep": {} }}"#, nested(200)));
}

#[test]
fn array_index_selector() {
    assert_parity("[2,0]", ARRAY);
    assert_parity("[0,0,0]", ARRAY);
    assert_parity("[4,2,0]", ARRAY);
    assert_parity("[0,9]", ARRAY);
    assert_parity("[0]", r#"{ "a": 1 }"#);
    assert_parity("[0,1]", r#"{ "a": 1 }"#);
}

#[test]
fn array_range_selector() {
    assert_parity("[0:2]", ARRAY);
    assert_parity("[2:0]", ARRAY);
    assert_parity("[0:0]", ARRAY);
    assert_parity("[:2]", ARRAY);
    assert_parity("[2:]", ARRAY);
    assert_parity("[:5]", ARRAY);
    assert_parity("[0:2]", "[]");
    assert_parity("[0:2]", "1");
}

#[test]
fn object_index_selector() {
    assert_parity("{2,0}", OBJECT);
    assert_parity("{4,2,0}", OBJECT);
    assert_parity("{0,0}", OBJECT);
    assert_parity("{4,2,10}", OBJECT);
    assert_parity("{0}", "{}");
    assert_parity("{0}", "[1, 2, 3]");
}

#[test]
fn object_range_selector() {
    assert_parity("{0:2}", OBJECT);
    assert_parity("{2:0}", OBJECT);
    assert_parity("{0:0}", OBJECT);
    assert_parity("{:2}", OBJECT);
    assert_parity("{4:}", OBJECT);
    assert_parity("{:5}", OBJECT);
    assert_parity("{0:2}", "{}");
    assert_parity("{0:2}", "1");
}

#[test]
fn multi_key_selector() {
    assert_parity(r#"{"a","b","c"}"#, OBJECT);
    assert_parity(r#"{"c","a","b"}"#, OBJECT);
    assert_parity(r#"{"w","a","t"}"#, OBJECT);
    assert_parity(r#"{"a","a"}"#, OBJECT);
    assert_parity(r#"{"a"}"#, "1");
    assert_parity(r#""o"{"z","a"}"#, r#"{ "o": { "z": 1, "a": 2, "m": 3 } }"#);
}

#[test]
fn keys_operator() {
    assert_parity("@", OBJECT);
    assert_parity("@", r#"{ "z": 1, "a": 2, "m": 3 }"#);
    assert_parity("@", ARRAY);
    assert_parity("@", "[]");
    assert_parity("@", "{}");
    assert_parity("@", "true");
    assert_parity("@", "42");
    assert_parity("@", r#""text""#);
    assert_parity("@", "null");
    assert_parity(r#""a""b""c"@"#, r#"{ "a": { "b": { "c": { "d": 1 } } } }"#);
}

#[test]
fn truncate_operator() {
    assert_parity("!", OBJECT);
    assert_parity("!", ARRAY);
    assert_parity("!", "true");
    assert_parity("!", "1");
    assert_parity("!", r#""x""#);
    assert_parity("!", "null");
    assert_parity(r#""a"!"#, r#"{ "a": { "b": 1 } }"#);
}

#[test]
fn groups() {
    assert_parity(r#""a","b","c""#, OBJECT);
    assert_parity(r#""a",[0],"c""#, r#"{ "a": 1, "arr": [9], "c": 3 }"#);
    assert_parity(r#""a"@,"b"!"#, r#"{ "a": { "x": 1 }, "b": [1, 2] }"#);
    assert_parity(r#""a"{"y","x"},"a"@"#, r#"{ "a": { "x": 1, "y": 2 } }"#);
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
    for literal in [
        "0",
        "-1",
        "18446744073709551615",
        "-9223372036854775808",
        "3.14",
        "1e10",
        "-0.0",
        "1234567890123456789",
        "0.1",
        "2.5e-3",
        "-0",
        "-0e2",
        "-0.5",
    ] {
        assert_parity(r#""n""#, &format!(r#"{{ "n": {literal} }}"#));
    }
    // Long decimal mantissa.
    assert_parity(
        r#""n""#,
        r#"{ "n": 1111110004366660488538014150.111111111111111111 }"#,
    );
    // `-0` inside a string must not trigger the fall back incorrectly.
    assert_parity(r#""s""#, r#"{ "s": "-0 degrees" }"#);
    // Integers past i64/u64 make simd-json reject the tape -> fall back.
    assert_parity(r#""n""#, r#"{ "n": 18446744073709551616 }"#);
    assert_parity(
        r#""s""#,
        r#"{ "s": "with \"quotes\" and \n newline and é 🦀" }"#,
    );
    assert_parity(r#""b""#, r#"{ "b": false }"#);
    assert_parity(r#""z""#, r#"{ "z": null }"#);
    assert_parity(r#""e""#, r#"{ "e": {} }"#);
    assert_parity(r#""e""#, r#"{ "e": [] }"#);
}

/// The hazardous numbers are looked for in what a query returns, not in what it
/// reads, so a `-0` or a long mantissa the query never selects must neither
/// change the result nor cost the fast path.
#[test]
fn hazardous_numbers_outside_the_selection() {
    let json = r#"{ "a": "kept", "b": -0, "c": 0.12345678901234567890, "d": 0 }"#;

    assert_parity(r#""a""#, json);
    assert_parity(r#""d""#, json);
    assert_parity(r#""b""#, json);
    assert_parity(r#""c""#, json);
    assert_parity(r#"{"a","d"}"#, json);
    assert_parity("@", json);
    assert_parity("!", json);

    // An integer zero selected out of input that holds a `-0` elsewhere is not
    // itself in danger, but the scan cannot tell them apart, so this only has
    // to stay correct.
    assert_parity(r#""d","a""#, json);

    // A lens comparing numbers decides the selection, so both hazards are
    // looked for up front there.
    assert_parity(r#"|={"n"=0}"#, r#"[{ "n": -0 }, { "n": 0 }, { "n": 1 }]"#);
    assert_parity(
        r#"|={"n"=1}"#,
        r#"[{ "n": 0.12345678901234567890 }, { "n": 1 }]"#,
    );
}

/// Keys carrying JSON escape sequences must resolve the same way on the tape
/// as they do on the `Value` oracle.
#[test]
fn escaped_keys() {
    assert_parity(r#""a\"b""#, r#"{ "a\"b": 1, "c": 2 }"#);
    assert_parity(r#""a\\b""#, r#"{ "a\\b": 1 }"#);
    assert_parity(r#""a\nb""#, r#"{ "a\nb": 1 }"#);
    assert_parity(r#""\u0041""#, r#"{ "A": 1 }"#);
    assert_parity(r#""\uD83D\uDE00""#, r#"{ "\uD83D\uDE00": 1 }"#);
    assert_parity(r#"{"a\"b","c"}"#, r#"{ "a\"b": 1, "c": 2 }"#);
    assert_parity(r#""missing\"key""#, r#"{ "a": 1 }"#);
}

/// Hazardous numbers reached through an error must fall back too: the error
/// embeds the value it was raised on and is compared whole.
#[test]
fn hazardous_numbers_inside_an_error() {
    assert_parity(r#""missing""#, r#"{ "b": -0 }"#);
    assert_parity(r#""missing""#, r#"{ "b": 0.12345678901234567890 }"#);
    assert_parity("[9]", r#"[-0, 1]"#);
    assert_parity(r#"{"missing","other"}"#, r#"{ "b": -0 }"#);
    assert_parity("[0:9]", r#"[-0]"#);
    assert_parity(r#""a"|>"b""#, r#"{ "a": { "b": -0 } }"#);
}

/// The one accepted departure from exact parity: simd-json and serde_json round
/// the last `f64` bit of some scientific-notation literals differently, and
/// telling a number's exponent from an `e` inside a string is too expensive to
/// scan for (see `numbers_may_diverge`). The tape is the correctly rounded side
/// of the disagreement, so the check is that the tape matches Rust's own
/// correctly rounded parse, and that the oracle stays within two ULP of it.
#[test]
fn scientific_notation_is_correctly_rounded_on_the_tape() {
    for literal in [
        "1222211e222",
        "1222211e30",
        "1222211e250",
        "1.5e12",
        "9.87e-40",
        "1e200",
        "1e10",
        "9e75",
        "1e-29",
        "21e-45",
        "755841178345336393e-23",
    ] {
        let json = format!(r#"{{ "n": {literal} }}"#);
        let query = r#""n""#;

        let oracle = runner::raw(query, &deserialize(&json)).unwrap();
        let lazy = lazy::raw(query, &mut json.as_bytes().to_vec()).unwrap();

        let (oracle, lazy) = (oracle.as_f64().unwrap(), lazy.as_f64().unwrap());
        let truth: f64 = literal.parse().unwrap();

        assert_eq!(
            lazy.to_bits(),
            truth.to_bits(),
            "{literal} is not correctly rounded on the tape"
        );

        let ulps = ((oracle.to_bits() as i128) - (lazy.to_bits() as i128)).abs();

        assert!(ulps <= 2, "{literal} differs by {ulps} ULP");
    }
}

#[test]
fn duplicate_keys() {
    // serde_json keeps the last value per key; the tape keeps every entry, so
    // object operators on such an object must delegate.
    let dup = r#"{ "a": 0, "b": 2, "c": 2, "c": 33 }"#;
    assert_parity(r#""c""#, dup);
    assert_parity("{2,0}", dup);
    assert_parity("{0:2}", dup);
    assert_parity(r#"{"c","a"}"#, dup);
    assert_parity("@", dup);
    assert_parity("!", dup);
    assert_parity(r#""o""#, r#"{ "o": { "x": 1, "x": 9 } }"#);
    assert_parity(r#""o""x""#, r#"{ "o": { "x": 1, "x": 9 } }"#);
    assert_parity("{5}", dup);
}

#[test]
fn key_order_preserved() {
    assert_parity(r#""o""#, r#"{ "o": { "z": 1, "y": 2, "a": 3, "m": 4 } }"#);
    assert_parity("{0:3}", r#"{ "z": 1, "y": 2, "a": 3, "m": 4, "b": 5 }"#);
    assert_parity(r#"{"m","z","a"}"#, r#"{ "z": 1, "y": 2, "a": 3, "m": 4 }"#);
    assert_parity(
        r#""o""nested""#,
        r#"{ "o": { "nested": { "d": 1, "c": 2, "b": 3, "a": 4 } } }"#,
    );
}

#[test]
fn pipe_operator() {
    let nested = r#"{ "a": [{ "b": { "c": 1 } }, { "b": { "c": 2 } }] }"#;
    assert_parity(r#""a"|>"b""c""#, nested);
    assert_parity(r#""a"|>"b""c"<|[1]"#, nested);
    assert_parity(r#""a"|>"b""#, nested);
    assert_parity(r#""a"|>"b"<|"#, nested);
    assert_parity(r#""a"|>"b""c"<|@"#, nested);

    // Pipe over a projection: `{name}` per row.
    let rows = r#"{ "rows": [{ "name": "a", "n": 1 }, { "name": "b", "n": 2 }, { "name": "c", "n": 3 }] }"#;
    assert_parity(r#""rows"|>{"name","n"}"#, rows);
    assert_parity(r#""rows"|>"name""#, rows);
    assert_parity(r#""rows"|>[0]"#, r#"{ "rows": [[1, 2], [3, 4]] }"#);
    assert_parity(r#""rows"|>"name"<|[2,0]"#, rows);

    // Pipe where an element has the wrong type, and pipe on a non-array.
    assert_parity(r#""r"|>"k""#, r#"{ "r": [{ "k": 1 }, 5] }"#);
    assert_parity(r#""r"|>"k""#, r#"{ "r": { "k": 1 } }"#);
    assert_parity("<|", r#"[1, 2]"#);
    assert_parity(r#""r"|>"missing""#, r#"{ "r": [{ "k": 1 }] }"#);

    // Pipe body that delegates (flatten inside the pipe).
    assert_parity(r#""r"|>..[0]"#, r#"{ "r": [[1, [2]], [3, [4]]] }"#);

    // Each body token is applied to every element before the next one, so a
    // failure is reported for the token the runner reaches first — here the key
    // missing from the last element, not the index applied to the first.
    assert_parity(
        r#""x"|>"k"[21]"#,
        r#"{ "x": [{ "k": 4 }, { "k": 8 }, { "i": 7 }] }"#,
    );
    assert_parity(r#""x"|>"k"@"#, r#"{ "x": [{ "k": 4 }, { "i": 7 }] }"#);

    // Empty pipe body, and a stray second `<|`.
    assert_parity(r#""r"|><|"#, r#"{ "r": [1, 2] }"#);
    assert_parity(r#""r"|>"k"<|<|"#, r#"{ "r": [{ "k": 1 }] }"#);
    assert_parity(r#""r"|><|[0]"#, r#"{ "r": [1, 2] }"#);

    // Piping into an empty array leaves the runner's `piped` flag set, so every
    // later token maps over nothing and the group yields `[]`.
    assert_parity(r#""r"|>"k""#, r#"{ "r": [] }"#);
    assert_parity(r#""r"|>"k"<|[0,2]"#, r#"{ "r": [] }"#);
    assert_parity(r#""r"|>"k"<|@"#, r#"{ "r": [] }"#);
    assert_parity(r#""r"|><|{"a"}"#, r#"{ "r": [] }"#);
    assert_parity(r#""r"|>"k"<|"missing""#, r#"{ "r": [] }"#);
}

#[test]
fn lens_selector() {
    let people = r#"[
        { "a": 1, "b": 2 },
        { "a": 2, "b": "some" },
        { "a": 2, "b": null },
        { "a": 2, "b": true }
    ]"#;
    assert_parity(r#"|={"a"}"#, people);
    assert_parity(r#"|={"a"=1}"#, people);
    assert_parity(r#"|={"a"=1, "a"=2}"#, people);
    assert_parity(r#"|={"a"=1, "b"=2}"#, people);
    assert_parity(r#"|={"a"=1, "b"="some"}"#, people);
    assert_parity(r#"|={"a"=1, "b"=true}"#, people);
    assert_parity(r#"|={"a"=1, "b"=null}"#, people);
    assert_parity(r#"|={"missing"}"#, people);
    assert_parity(r#"|={"a"}"#, "[]");
    assert_parity(r#"|={"a"}"#, "1");
    assert_parity(r#""x"|={"k"}"#, r#"{ "x": [{ "k": 1 }, { "j": 2 }] }"#);
    assert_parity(
        r#"|={"a""b""c"=2}"#,
        r#"[{ "a": { "b": { "c": 2 } } }, { "a": { "b": { "c": 3 } } }]"#,
    );
    // A lens body accepts every selector `parse_lens_key` allows.
    assert_parity(r#"|={[0]}"#, r#"[[1, 2], "x", []]"#);
    assert_parity(r#"|={[0]=1}"#, r#"[[1, 2], [3, 4]]"#);
    assert_parity(r#"|={[0:1]}"#, r#"[[1, 2], "x"]"#);
    assert_parity(r#"|={{"a","b"}}"#, r#"[{ "a": 1, "b": 2 }, { "a": 1 }]"#);
    assert_parity(r#"|={{0}}"#, r#"[{ "a": 1 }, "x"]"#);
    assert_parity(r#"|={{0:0}}"#, r#"[{ "a": 1 }, "x"]"#);
    // A key that merely looks like the flatten operator.
    assert_parity(r#"|={".."}"#, r#"[{ "..": 1 }, { "x": 2 }]"#);
    // Lens after a pipe.
    assert_parity(
        r#""g"|>"rows"<||={"k"=1}"#,
        r#"{ "g": [{ "rows": [{ "k": 1 }, { "k": 2 }] }] }"#,
    );
}

#[test]
fn delegated_operators() {
    // Flatten and nested pipes are not on the tape path: `lazy` delegates.
    assert_parity("..", r#"[1, [2], [[3]]]"#);
    assert_parity(r#""a"..[0]"#, r#"{ "a": [1, [2], [[3]]] }"#);
    assert_parity("..", r#"{ "a": { "c": false }, "b": { "d": { "e": 1 } } }"#);
    assert_parity(r#""a"|>"b"|>"c""#, r#"{ "a": [{ "b": [{ "c": 1 }] }] }"#);
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

/// Every document in the input is evaluated, and the tape-per-document path in
/// `raw_all` agrees with running the `Value` runner over each document.
#[track_caller]
fn assert_all_parity(query: &str, json: &str) {
    let documents = deserialize_all(json);

    let oracle: Result<Vec<Value>, _> = documents
        .iter()
        .map(|document| runner::raw(query, document))
        .collect();
    let lazy = lazy::raw_all(query, &mut json.as_bytes().to_vec());

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
fn every_document_is_evaluated() {
    assert_all_parity(r#""a""#, r#"{ "a": 1 }"#);
    assert_all_parity(r#""a""#, r#"{ "a": 1 }{ "a": 2 }"#);
    assert_all_parity(r#""a""#, "{ \"a\": 1 }\n{ \"a\": 2 }\n{ \"a\": 3 }\n");
    assert_all_parity(r#""a""#, "{\n  \"a\": 1\n}\n{\n  \"a\": 2\n}\n");
    assert_all_parity("[0]", "[1][2][3][4][5][6][7][8][9][10]");
    assert_all_parity(r#"|>{"n"}"#, r#"[{ "n": 1 }][{ "n": 2 }]"#);
    assert_all_parity(r#""a"@"#, r#"{ "a": { "y": 1 } }{ "a": { "x": 2 } }"#);
    // A document that the tape declines still matches: deep nesting, a bare -0,
    // and a long decimal mantissa each fall back on their own.
    assert_all_parity(r#""n""#, r#"{ "n": -0 }{ "n": 1 }"#);
    assert_all_parity(
        r#""n""#,
        r#"{ "n": 1 }{ "n": 1111110004366660488538014150.111111111111111111 }"#,
    );
    // A failing document fails the whole run.
    assert_all_parity(r#""a""#, r#"{ "a": 1 }{ "b": 2 }"#);
    assert_all_parity(r#""a""#, r#"{ "a": 1 }[2]"#);
}

/// The boundaries between documents come out of the structural positions the
/// failed tape build already found, and whitespace can sit anywhere around them.
/// Top-level scalars and strings are read the slower way instead, so they have
/// to keep working too.
#[test]
fn document_boundaries_in_every_shape() {
    assert_all_parity(r#""a""#, r#"   { "a": 1 }   { "a": 2 }   "#);
    assert_all_parity(r#""a""#, "\t{ \"a\": 1 }\r\n\r\n{ \"a\": 2 }\n");
    assert_all_parity(r#""a""#, r#"{ "a": "}{" }{ "a": "][" }"#);
    assert_all_parity(r#""a""#, r#"{ "a": "\"}\"" }{ "a": 2 }"#);
    assert_all_parity("[0]", r#"[[1]][[2]]"#);
    assert_all_parity("@", r#"{ "a": 1 }{ "b": 2, "c": 3 }"#);

    // Top-level scalars, strings, booleans and nulls as whole documents.
    assert_all_parity("@", "1 2 3");
    assert_all_parity("@", r#""a" "b""#);
    assert_all_parity("@", "true false null");
    assert_all_parity("@", r#"{ "a": 1 } 2"#);
    assert_all_parity("@", r#"1 { "a": 2 }"#);
}

#[test]
fn multi_document_input_rejects_trailing_junk() {
    for json in [r#"{ "a": 1 } junk"#, r#"{ "a": 1 }{"#, "", "   "] {
        assert_eq!(
            lazy::raw_all(r#""a""#, &mut json.as_bytes().to_vec()),
            Err(jql_runner::errors::JqlRunnerError::DeserializationError),
            "expected a deserialization error for `{json}`"
        );
    }
}

/// Reads every document, growing the stack the way the binary does so that
/// deeply nested fixtures build an oracle instead of hitting serde_json's
/// default recursion limit.
fn deserialize_all(json: &str) -> Vec<Value> {
    let mut deserializer = serde_json::Deserializer::from_str(json);

    deserializer.disable_recursion_limit();

    let mut documents = Vec::new();

    while let Ok(document) = Value::deserialize(serde_stacker::Deserializer::new(&mut deserializer))
    {
        documents.push(document);
    }

    assert!(!documents.is_empty(), "no document parsed from `{json}`");

    documents
}

/// Several documents nested past what the tape accepts: the range scan still
/// finds them, because serde_json skips with an explicit stack rather than by
/// recursing, and each one then falls back on its own.
#[test]
fn deeply_nested_multi_document_input_is_read() {
    let json = format!("{}{}", nested(1100), nested(1100));
    let results =
        lazy::raw_all("[0]", &mut json.as_bytes().to_vec()).expect("both documents should be read");

    assert_eq!(results.len(), 2);
}
