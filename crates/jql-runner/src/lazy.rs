use std::{
    collections::HashSet,
    num::NonZeroUsize,
};

use jql_parser::{
    group::split,
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
    prelude::{
        TypedValue,
        ValueAsScalar,
        ValueType,
    },
    tape::{
        Object as TapeObject,
        Value as TapeValue,
    },
};

use crate::{
    errors::JqlRunnerError,
    runner,
};

/// Takes a raw query and a slice of JSON bytes.
/// Returns a JSON `Value` or an error.
///
/// Selection queries (key, index, range, multi-key, keys `@`, truncate `!` and a
/// single pipe `|> … <|`) are evaluated against a simd-json tape, materializing
/// only the selected subtrees. Queries using the flatten or lens operators or
/// nested pipes, and any input the tape builder rejects (nested beyond
/// simd-json's limit, trailing data, malformed), fall back to the `Value`-based
/// [`runner`], preserving its exact behavior.
///
/// # Errors
///
/// Returns a `JqlRunnerError` on failure.
pub fn raw(query: &str, json: &[u8]) -> Result<Value, JqlRunnerError> {
    if query.is_empty() {
        return Err(JqlRunnerError::EmptyQueryError);
    }

    let tokens = parse(query)?;

    if can_lazy(&tokens) && !numbers_may_diverge(json) {
        // `to_tape` parses in place, so give it a copy — the original bytes must
        // stay pristine for the fallback deserializer.
        let mut scratch = json.to_vec();

        if let Ok(tape) = simd_json::to_tape(&mut scratch) {
            return eval(&tokens, tape.as_value());
        }
    }

    runner::token(&tokens, &deserialize(json)?)
}

/// A digit count this far past a decimal point is already well over the 16 that
/// matters, and caps the work on adversarial input.
const DIGIT_SCAN_CAP: usize = 24;

/// Whether `json` has a number literal that simd-json and serde_json could parse
/// to different `f64` bits, so the tape path must be skipped:
///
/// - `-0`: serde_json makes it the float `-0.0`, simd-json the integer `0`.
/// - a fractional literal with 16+ significant digits: the two parsers can round
///   the last bit differently. Only `.`-bearing literals are scanned — that is
///   what serializers emit for non-integers, so real input stays exact; the rare
///   `123…e10` form is left to the caller's tolerant comparison.
///
/// Plain integers are fine: exact both ways, or simd-json rejects them and the
/// tape build fails anyway. The scan is loose — a false positive inside a string
/// only costs a fall back to the runner, never correctness.
fn numbers_may_diverge(json: &[u8]) -> bool {
    let bare_negative_zero = memchr::memchr_iter(b'-', json).any(|index| {
        json.get(index + 1) == Some(&b'0')
            && !matches!(json.get(index + 2), Some(b'0'..=b'9' | b'.' | b'e' | b'E'))
    });

    bare_negative_zero
        || memchr::memchr_iter(b'.', json).any(|dot| {
            // A digit must border the point on the left for this to be a number.
            if dot == 0 || !json[dot - 1].is_ascii_digit() {
                return false;
            }

            let before = json[..dot]
                .iter()
                .rev()
                .take(DIGIT_SCAN_CAP)
                .take_while(|byte| byte.is_ascii_digit())
                .count();
            let after = json[dot + 1..]
                .iter()
                .take(DIGIT_SCAN_CAP)
                .take_while(|byte| byte.is_ascii_digit())
                .count();

            before + after >= 16
        })
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

/// Returns `true` when the query only uses operators the tape evaluator handles
/// and its first selector narrows the input (so building the tape pays off).
fn can_lazy(tokens: &[Token]) -> bool {
    let unsupported =
        |token: &&Token| matches!(token, Token::FlattenOperator | Token::LensSelector(_));

    // A leading flatten/lens/pipe-out builds the tape only to discard it.
    let usable_first = tokens
        .iter()
        .find(|token| !matches!(token, Token::GroupSeparator))
        .is_some_and(|token| {
            !matches!(
                token,
                Token::FlattenOperator | Token::LensSelector(_) | Token::PipeOutOperator
            )
        });

    // Nested pipe-ins have subtle semantics; leave more than one to the runner.
    let pipe_ins = tokens
        .iter()
        .filter(|token| matches!(token, Token::PipeInOperator))
        .count();

    usable_first && pipe_ins <= 1 && !tokens.iter().any(|token| unsupported(&token))
}

/// Evaluates the token groups against the tape root, mirroring
/// [`runner::token`]: a single group yields its value, multiple groups yield an
/// array of per-group values.
fn eval(tokens: &[Token], root: TapeValue) -> Result<Value, JqlRunnerError> {
    let groups = split(tokens);

    if groups.len() == 1 {
        return eval_group(&groups[0], root);
    }

    groups
        .iter()
        .map(|group| eval_group(group, root))
        .collect::<Result<Vec<Value>, _>>()
        .map(Value::Array)
}

/// A position in the evaluation: still a view into the tape, or a value the
/// previous operator produced.
enum Cursor<'tape, 'input> {
    Tape(TapeValue<'tape, 'input>),
    Owned(Value),
}

impl Cursor<'_, '_> {
    fn into_value(self) -> Value {
        match self {
            Cursor::Tape(value) => materialize(value),
            Cursor::Owned(value) => value,
        }
    }
}

/// Evaluates a group's tokens starting from the tape root.
fn eval_group(tokens: &[&Token], root: TapeValue) -> Result<Value, JqlRunnerError> {
    eval_tokens(tokens, Cursor::Tape(root))
}

/// Folds tokens over a cursor. `|>` maps the tokens up to the matching `<|` (or
/// the end) over each element; `<|` without a preceding `|>` is an error. Once an
/// operator the tape path does not implement is reached, the remaining tokens are
/// handed to [`runner`].
fn eval_tokens<'tape, 'input>(
    tokens: &[&Token],
    mut cursor: Cursor<'tape, 'input>,
) -> Result<Value, JqlRunnerError> {
    let mut index = 0;

    while let Some(&token) = tokens.get(index) {
        match token {
            Token::PipeInOperator => {
                let elements = pipe_elements(cursor)?;
                let rest = &tokens[index + 1..];
                let body_len = rest
                    .iter()
                    .position(|token| matches!(token, Token::PipeOutOperator))
                    .unwrap_or(rest.len());

                let mapped = elements
                    .into_iter()
                    .map(|element| eval_tokens(&rest[..body_len], element))
                    .collect::<Result<Vec<Value>, _>>()?;

                cursor = Cursor::Owned(Value::Array(mapped));
                index += 1 + body_len;

                // Consume the matching `<|`, if any.
                if matches!(tokens.get(index), Some(Token::PipeOutOperator)) {
                    index += 1;
                }
            }

            Token::PipeOutOperator => return Err(JqlRunnerError::PipeOutError),

            _ => {
                cursor = match cursor {
                    Cursor::Owned(value) => {
                        return runner::group_runner(&tokens[index..], &value);
                    }
                    Cursor::Tape(value) => match step(value, token)? {
                        Some(next) => next,
                        None => {
                            return runner::group_runner(&tokens[index..], &materialize(value));
                        }
                    },
                };

                index += 1;
            }
        }
    }

    Ok(cursor.into_value())
}

/// Splits a cursor into per-element cursors for `|>`, or errors if it is not an
/// array (mirroring `matcher`'s `PipeInError`).
fn pipe_elements<'tape, 'input>(
    cursor: Cursor<'tape, 'input>,
) -> Result<Vec<Cursor<'tape, 'input>>, JqlRunnerError> {
    match cursor {
        Cursor::Tape(value) => value.as_array().map_or_else(
            || Err(JqlRunnerError::PipeInError(materialize(value))),
            |array| Ok(array.iter().map(Cursor::Tape).collect()),
        ),
        Cursor::Owned(Value::Array(items)) => Ok(items.into_iter().map(Cursor::Owned).collect()),
        Cursor::Owned(other) => Err(JqlRunnerError::PipeInError(other)),
    }
}

/// Applies one token to a tape view. `Ok(None)` means the tape path does not
/// implement this token and the caller should fall back to [`runner`].
#[allow(clippy::too_many_lines)]
fn step<'tape, 'input>(
    value: TapeValue<'tape, 'input>,
    token: &Token,
) -> Result<Option<Cursor<'tape, 'input>>, JqlRunnerError> {
    // serde_json keeps only the last value for a repeated key; the tape keeps
    // every entry. Any object operator on such an object would diverge, so
    // delegate: `materialize` collapses duplicates the same way serde_json does.
    if operates_on_object(token)
        && let Some(object) = value.as_object()
        && has_duplicate_keys(&object)
    {
        return Ok(None);
    }

    let cursor = match token {
        Token::KeySelector(key) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            Cursor::Tape(
                object
                    .get(*key)
                    .ok_or_else(|| JqlRunnerError::KeyNotFoundError {
                        key: (*key).to_string(),
                        parent: materialize(value),
                    })?,
            )
        }

        Token::ArrayIndexSelector(indexes) if indexes.len() == 1 => {
            let index: usize = indexes[0].into();

            Cursor::Tape(value.get_idx(index).ok_or_else(|| {
                JqlRunnerError::IndexOutOfBoundsError {
                    index,
                    parent: materialize(value),
                }
            })?)
        }

        Token::ArrayIndexSelector(indexes) => {
            let selected = indexes
                .iter()
                .map(|index| {
                    let index: usize = (*index).into();

                    value.get_idx(index).map(materialize).ok_or_else(|| {
                        JqlRunnerError::IndexOutOfBoundsError {
                            index,
                            parent: materialize(value),
                        }
                    })
                })
                .collect::<Result<Vec<Value>, _>>()?;

            Cursor::Owned(Value::Array(selected))
        }

        Token::ArrayRangeSelector(range) => {
            let Some(array) = value.as_array() else {
                return Err(JqlRunnerError::InvalidArrayError(materialize(value)));
            };

            let Some(len) = NonZeroUsize::new(array.len()) else {
                return Ok(Some(Cursor::Owned(Value::Array(Vec::new()))));
            };

            let (start, end) = range.to_boundaries(len);

            if start >= len.get() || end >= len.get() {
                return Err(JqlRunnerError::RangeOutOfBoundsError {
                    start,
                    end,
                    parent: materialize(value),
                });
            }

            let (low, high, reverse) = span(start, end);
            let mut selected: Vec<Value> = array
                .iter()
                .skip(low)
                .take(high - low + 1)
                .map(materialize)
                .collect();

            if reverse {
                selected.reverse();
            }

            Cursor::Owned(Value::Array(selected))
        }

        Token::ObjectIndexSelector(indexes) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            if object.is_empty() {
                return Ok(Some(Cursor::Owned(Value::Object(Map::new()))));
            }

            let Some(&max) = indexes.iter().max() else {
                // The parser never emits an empty index list.
                return Ok(None);
            };
            let max: usize = max.into();

            if max >= object.len() {
                return Err(JqlRunnerError::IndexOutOfBoundsError {
                    index: max,
                    parent: materialize(value),
                });
            }

            let entries: Vec<(&str, TapeValue)> = object.iter().collect();
            let mut map = Map::with_capacity(indexes.len());

            for index in indexes {
                let index: usize = (*index).into();

                if let Some(&(key, child)) = entries.get(index) {
                    map.insert(key.to_string(), materialize(child));
                }
            }

            Cursor::Owned(Value::Object(map))
        }

        Token::ObjectRangeSelector(range) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            let Some(len) = NonZeroUsize::new(object.len()) else {
                return Ok(Some(Cursor::Owned(Value::Object(Map::new()))));
            };

            let (start, end) = range.to_boundaries(len);

            if start >= len.get() || end >= len.get() {
                return Err(JqlRunnerError::RangeOutOfBoundsError {
                    start,
                    end,
                    parent: materialize(value),
                });
            }

            let (low, high, reverse) = span(start, end);
            let mut entries: Vec<(&str, TapeValue)> =
                object.iter().skip(low).take(high - low + 1).collect();

            if reverse {
                entries.reverse();
            }

            Cursor::Owned(Value::Object(
                entries
                    .into_iter()
                    .map(|(key, child)| (key.to_string(), materialize(child)))
                    .collect(),
            ))
        }

        Token::MultiKeySelector(keys) => {
            let Some(object) = value.as_object() else {
                return Err(JqlRunnerError::InvalidObjectError(materialize(value)));
            };

            let mut missing: Vec<String> = keys
                .iter()
                .filter(|key| object.get(**key).is_none())
                .map(|key| (*key).to_string())
                .collect();

            if !missing.is_empty() {
                missing.sort();

                return Err(JqlRunnerError::MultiKeyNotFoundError {
                    keys: missing,
                    parent: materialize(value),
                });
            }

            // Every key is present (checked above); any miss is simply skipped.
            Cursor::Owned(Value::Object(
                keys.iter()
                    .filter_map(|key| {
                        object
                            .get(*key)
                            .map(|child| ((*key).to_string(), materialize(child)))
                    })
                    .collect(),
            ))
        }

        Token::KeyOperator => Cursor::Owned(if let Some(object) = value.as_object() {
            let mut keys: Vec<&str> = object.keys().collect();

            keys.sort_unstable();

            Value::Array(
                keys.into_iter()
                    .map(|key| Value::String(key.to_string()))
                    .collect(),
            )
        } else if let Some(array) = value.as_array() {
            Value::Array((0..array.len()).map(Value::from).collect())
        } else {
            materialize(value)
        }),

        Token::TruncateOperator => Cursor::Owned(match value.value_type() {
            ValueType::Array => Value::Array(Vec::new()),
            ValueType::Object => Value::Object(Map::new()),
            _ => materialize(value),
        }),

        // `GroupSeparator` is removed by `split` and the pipe operators are
        // handled by `eval_tokens`, so only flatten and lens reach here; both
        // delegate.
        Token::GroupSeparator
        | Token::FlattenOperator
        | Token::LensSelector(_)
        | Token::PipeInOperator
        | Token::PipeOutOperator => return Ok(None),
    };

    Ok(Some(cursor))
}

/// Whether `token` reads an object by key or position (and so is sensitive to
/// how duplicate keys are handled).
fn operates_on_object(token: &Token) -> bool {
    matches!(
        token,
        Token::KeySelector(_)
            | Token::MultiKeySelector(_)
            | Token::ObjectIndexSelector(_)
            | Token::ObjectRangeSelector(_)
            | Token::KeyOperator
    )
}

/// Whether any key appears more than once in `object`.
fn has_duplicate_keys(object: &TapeObject) -> bool {
    let mut seen = HashSet::with_capacity(object.len());

    object.keys().any(|key| !seen.insert(key))
}

/// Turns a directional `(start, end)` into an ordered `(low, high, reverse)`,
/// mirroring the runner's natural-order handling.
fn span(start: usize, end: usize) -> (usize, usize, bool) {
    if start < end {
        (start, end, false)
    } else {
        (end, start, true)
    }
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
        raw(query, json.as_bytes())
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
    fn check_selection_operators() {
        assert_eq!(
            run("[2,0]", r#"["a", "b", "c", "d"]"#),
            Ok(json!(["c", "a"]))
        );
        assert_eq!(
            run("[2:0]", r#"["a", "b", "c"]"#),
            Ok(json!(["c", "b", "a"]))
        );
        assert_eq!(
            run(r#"{"c","a"}"#, r#"{ "a": 1, "b": 2, "c": 3 }"#),
            Ok(json!({ "c": 3, "a": 1 }))
        );
        assert_eq!(
            run(r#""a"@"#, r#"{ "a": { "y": 1, "x": 2 } }"#),
            Ok(json!(["x", "y"]))
        );
        assert_eq!(run(r#""a"!"#, r#"{ "a": [1, 2, 3] }"#), Ok(json!([])));
    }

    #[test]
    fn check_groups() {
        assert_eq!(
            run(r#""a","b""#, r#"{ "a": 1, "b": 2 }"#),
            Ok(json!([1, 2]))
        );
    }

    #[test]
    fn check_errors() {
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
    fn check_delegates_flatten() {
        assert_eq!(
            run(r#""a"..[0]"#, r#"{ "a": [1, [2], [[3]]] }"#),
            Ok(json!(1))
        );
    }

    #[test]
    fn check_empty_query() {
        assert_eq!(run("", "{}"), Err(JqlRunnerError::EmptyQueryError));
    }
}
