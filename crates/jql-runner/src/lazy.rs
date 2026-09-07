use jql_parser::{
    parser::parse,
    tokens::Token,
};
use serde::Deserialize;
use serde_json::{
    Map,
    Number,
    Value,
};
use simd_json::{
    prelude::ValueAsScalar,
    tape::Value as TapeValue,
};

use crate::{
    errors::JqlRunnerError,
    runner,
};

/// Takes a raw query and a mutable slice of JSON bytes.
/// Returns a JSON `Value` or an error.
///
/// When the query is a pure drill-down (a single group of key and single-index
/// selectors) the input is scanned into a simd-json tape and only the selected
/// subtree is materialized. Every other query, and any input the tape builder
/// rejects (deeply nested beyond simd-json's limit, trailing data, malformed),
/// falls back to the `Value`-based [`runner`], preserving its exact behavior.
///
/// # Errors
///
/// Returns a `JqlRunnerError` on failure.
pub fn raw(query: &str, json: &mut [u8]) -> Result<Value, JqlRunnerError> {
    if query.is_empty() {
        return Err(JqlRunnerError::EmptyQueryError);
    }

    let tokens = parse(query)?;

    if is_drill_down(&tokens)
        && let Ok(tape) = simd_json::to_tape(json)
    {
        return navigate(&tokens, tape.as_value());
    }

    runner::token(&tokens, &deserialize(json)?)
}

/// Deserializes JSON bytes into a `Value`, growing the stack for deeply nested
/// input. Parses from `&str` so serde_json can slice string values without
/// re-validating UTF-8, matching the pre-tape deserialization path.
fn deserialize(json: &[u8]) -> Result<Value, JqlRunnerError> {
    let json = str::from_utf8(json).map_err(|_| JqlRunnerError::DeserializationError)?;

    let mut deserializer = serde_json::Deserializer::from_str(json);

    deserializer.disable_recursion_limit();

    Value::deserialize(serde_stacker::Deserializer::new(&mut deserializer))
        .map_err(|_| JqlRunnerError::DeserializationError)
}

/// Returns `true` when every token is a key selector or a single-index array
/// selector and the query has a single group.
fn is_drill_down(tokens: &[Token]) -> bool {
    !tokens.is_empty()
        && tokens.iter().all(|token| match token {
            Token::KeySelector(_) => true,
            Token::ArrayIndexSelector(indexes) => indexes.len() == 1,
            _ => false,
        })
}

/// Walks the drill-down tokens over the tape, materializing the final subtree.
fn navigate(tokens: &[Token], root: TapeValue) -> Result<Value, JqlRunnerError> {
    let mut current = root;

    for token in tokens {
        current = match token {
            Token::KeySelector(key) => {
                if current.as_object().is_none() {
                    return Err(JqlRunnerError::InvalidObjectError(materialize(current)));
                }

                current
                    .get(*key)
                    .ok_or_else(|| JqlRunnerError::KeyNotFoundError {
                        key: (*key).to_string(),
                        parent: materialize(current),
                    })?
            }
            Token::ArrayIndexSelector(indexes) => {
                let index: usize = indexes[0].into();

                current
                    .get_idx(index)
                    .ok_or_else(|| JqlRunnerError::IndexOutOfBoundsError {
                        index,
                        parent: materialize(current),
                    })?
            }
            // `is_drill_down` guarantees no other variant.
            _ => unreachable!(),
        };
    }

    Ok(materialize(current))
}

/// Recursively converts a tape `Value` into a `serde_json::Value`, preserving
/// document key order.
fn materialize(value: TapeValue) -> Value {
    if let Some(object) = value.as_object() {
        let mut map = Map::with_capacity(object.len());

        for (key, child) in &object {
            map.insert(key.to_string(), materialize(child));
        }

        return Value::Object(map);
    }

    if let Some(array) = value.as_array() {
        return Value::Array(array.iter().map(materialize).collect());
    }

    if value.as_null().is_some() {
        return Value::Null;
    }

    if let Some(boolean) = value.as_bool() {
        return Value::Bool(boolean);
    }

    if let Some(unsigned) = value.as_u64() {
        return Value::Number(unsigned.into());
    }

    if let Some(signed) = value.as_i64() {
        return Value::Number(signed.into());
    }

    if let Some(float) = value.as_f64() {
        return Number::from_f64(float).map_or(Value::Null, Value::Number);
    }

    // Remaining case: a string.
    value
        .as_str()
        .map_or(Value::Null, |string| Value::String(string.to_string()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::raw;
    use crate::errors::JqlRunnerError;

    fn run(query: &str, json: &str) -> Result<serde_json::Value, JqlRunnerError> {
        raw(query, &mut json.to_string().into_bytes())
    }

    #[test]
    fn check_drill_down() {
        assert_eq!(run(r#""a""b""#, r#"{ "a": { "b": 2 } }"#), Ok(json!(2)));
        assert_eq!(
            run(r#""a"[1]"c""#, r#"{ "a": [{ "c": 0 }, { "c": 1 }] }"#),
            Ok(json!(1))
        );
    }

    #[test]
    fn check_drill_down_errors() {
        assert_eq!(
            run(r#""b""#, r#"{ "a": 1 }"#),
            Err(JqlRunnerError::KeyNotFoundError {
                key: "b".to_string(),
                parent: json!({ "a": 1 }),
            })
        );
        assert_eq!(
            run("[1]", r#"["a"]"#),
            Err(JqlRunnerError::IndexOutOfBoundsError {
                index: 1,
                parent: json!(["a"]),
            })
        );
    }

    #[test]
    fn check_delegates_non_drill_down() {
        assert_eq!(
            run(r#""a","b""#, r#"{ "a": 1, "b": 2 }"#),
            Ok(json!([1, 2]))
        );
    }

    #[test]
    fn check_empty_query() {
        assert_eq!(run("", "{}"), Err(JqlRunnerError::EmptyQueryError));
    }
}
