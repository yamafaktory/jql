use jql_parser::{
    group::split,
    parser::parse,
    tokens::Token,
};
use rayon::prelude::*;
use serde_json::{
    json,
    Value,
};

use crate::{
    array::{
        get_array_indexes,
        get_array_lenses,
        get_array_range,
        get_flattened_array,
    },
    errors::JqlRunnerError,
    object::{
        get_flattened_object,
        get_object_indexes,
        get_object_key,
        get_object_multi_key,
        get_object_range,
    },
};

/// Takes a raw input as a slice string to parse and a reference of a JSON
/// `Value`.
/// Returns a JSON `Value`.
///
/// # Errors
///
/// Returns a `JqlRunnerError` on failure.
pub fn raw(input: &str, json: &Value) -> Result<Value, JqlRunnerError> {
    if input.is_empty() {
        return Err(JqlRunnerError::EmptyQueryError);
    }

    let tokens = parse(input)?;

    token(&tokens, json)
}

/// Takes a slice of `Tokens` to parse and a reference of a JSON
/// `Value`.
/// Returns a JSON `Value`.
///
/// # Errors
///
/// Returns a `JqlRunnerError` on failure.
pub fn token(tokens: &[Token], json: &Value) -> Result<Value, JqlRunnerError> {
    let groups = split(tokens);

    let result = groups
        .par_iter()
        .try_fold_with(vec![], |mut acc: Vec<Value>, group| {
            acc.push(group_runner(group, json)?);

            Ok::<Vec<Value>, JqlRunnerError>(acc)
        })
        .try_reduce(Vec::new, |mut a, b| {
            a.extend(b);

            Ok(a)
        });

    result.map(|group| {
        if groups.len() == 1 {
            json!(group[0])
        } else {
            json!(group)
        }
    })
}

/// Takes a slice of references of `Token` and a reference of a JSON `Value`.
/// Returns a JSON `Value` or an error.
/// Note: the `GroupSeparator` enum variant is unreachable at this point since
/// it has been filtered out by any of the public `runner` functions.
fn group_runner(tokens: &[&Token], json: &Value) -> Result<Value, JqlRunnerError> {
    tokens
        .iter()
        // At this level we can use rayon since every token is applied
        // sequentially.
        .try_fold((json.clone(), false), |mut outer_acc, &token| {
            if outer_acc.1 {
                let result = outer_acc
                    .0
                    .as_array_mut()
                    // We can safely unwrap since `outer_acc.1` is truthy.
                    .unwrap()
                    .par_iter()
                    .try_fold_with(
                        (vec![], outer_acc.1),
                        |mut inner_acc: (Vec<Value>, bool), inner_value| {
                            let result = matcher((inner_value.clone(), outer_acc.1), token)?;

                            inner_acc.0.push(result.0);
                            inner_acc.1 = result.1;

                            Ok::<(Vec<Value>, bool), JqlRunnerError>(inner_acc)
                        },
                    )
                    .try_reduce(
                        || (vec![], false),
                        |mut a, b| {
                            a.0.extend(b.0);

                            Ok((a.0, b.1))
                        },
                    )?;

                Ok((json!(result.0), result.1))
            } else {
                matcher(outer_acc, token)
            }
        })
        // Drop the `pipe` boolean flag.
        .map(|(value, _)| value)
}

/// Internal matcher consumed by the `group_runner` to apply a selection based
/// on the provided mutable JSON `Value` and the reference of a `Token`.
/// A `piped` flag is used to keep track of the pipe operators.
fn matcher(
    (mut acc, mut piped): (Value, bool),
    token: &Token,
) -> Result<(Value, bool), JqlRunnerError> {
    let result = match token {
        Token::ArrayIndexSelector(indexes) => get_array_indexes(indexes, &acc),
        Token::ArrayRangeSelector(range) => get_array_range(range, &mut acc),
        Token::FlattenOperator => match acc {
            Value::Array(_) => get_flattened_array(&acc),
            Value::Object(_) => Ok(get_flattened_object(&acc)),
            _ => Err(JqlRunnerError::FlattenError(acc)),
        },
        Token::GroupSeparator => unreachable!(),
        Token::KeySelector(key) => get_object_key(key, &acc),
        Token::LensSelector(lenses) => get_array_lenses(lenses, &mut acc),
        Token::MultiKeySelector(keys) => get_object_multi_key(keys, &mut acc),
        Token::ObjectIndexSelector(indexes) => get_object_indexes(indexes, &mut acc),
        Token::ObjectRangeSelector(range) => get_object_range(range, &mut acc),
        Token::PipeInOperator => {
            if !acc.is_array() {
                return Err(JqlRunnerError::PipeInError(acc));
            }

            piped = true;

            Ok(acc)
        }
        Token::PipeOutOperator => {
            if !piped {
                return Err(JqlRunnerError::PipeOutError);
            }

            piped = false;

            Ok(acc)
        }
        Token::TruncateOperator => match acc {
            Value::Array(_) => Ok(json!([])),
            Value::Object(_) => Ok(json!({})),
            Value::Bool(_) | Value::Number(_) | Value::String(_) | Value::Null => Ok(acc),
        },
    };

    result.map(|value| (value, piped))
}

#[cfg(test)]
mod tests {
    use jql_parser::{
        errors::JqlParserError,
        tokens::{
            Token,
            View,
        },
    };
    use serde_json::json;

    use super::raw;
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_runner_empty_input_error() {
        assert_eq!(raw("", &json!("")), Err(JqlRunnerError::EmptyQueryError));
    }

    #[test]
    fn check_runner_parsing_error() {
        assert_eq!(
            raw(r#""a"b"#, &json!({ "a": 1 })),
            Err(JqlRunnerError::ParsingError(JqlParserError::ParsingError {
                tokens: [Token::KeySelector("a")].stringify(),
                unparsed: "b".to_string(),
            }))
        );
    }

    #[test]
    fn check_runner_no_key_found_error() {
        let parent = json!({ "a": 1 });

        assert_eq!(
            raw(r#""b""#, &parent),
            Err(JqlRunnerError::KeyNotFoundError {
                key: "b".to_string(),
                parent
            })
        );
    }

    #[test]
    fn check_runner_index_not_found_error() {
        let parent = json!(["a"]);

        assert_eq!(
            raw("[1]", &parent),
            Err(JqlRunnerError::IndexOutOfBoundsError { index: 1, parent })
        );
    }

    #[test]
    fn check_runner_success() {
        assert_eq!(
            raw(r#""a","b""#, &json!({ "a": 1, "b": 2 })),
            Ok(json!([1, 2]))
        );
        assert_eq!(raw(r#""a""b""#, &json!({ "a": { "b": 2 } })), Ok(json!(2)));
        assert_eq!(
            raw("[4,2,0]", &json!(["a", "b", "c", "d", "e"])),
            Ok(json!(["e", "c", "a"]))
        );
    }

    #[test]
    fn check_pipes() {
        let value = json!({ "a": [{ "b": { "c": 1 } }, { "b": { "c": 2 }}]});

        assert_eq!(raw(r#""a"|>"b""c"<|[1]"#, &value), Ok(json!([2])));
    }

    #[test]
    fn check_truncate() {
        assert_eq!(raw(r#""a"!"#, &json!({ "a": [1, 2, 3] })), Ok(json!([])));
        assert_eq!(raw(r#""a"!"#, &json!({ "a": { "b": 1 } })), Ok(json!({})));
        assert_eq!(raw(r#""a"!"#, &json!({ "a": true })), Ok(json!(true)));
        assert_eq!(raw(r#""a"!"#, &json!({ "a": 1 })), Ok(json!(1)));
        assert_eq!(raw(r#""a"!"#, &json!({ "a": "b" })), Ok(json!("b")));
        assert_eq!(raw(r#""a"!"#, &json!({ "a": null })), Ok(json!(null)));
        assert_eq!(raw("!", &json!({ "a": null })), Ok(json!({})));
        assert_eq!(
            raw(r#""a"!"b""#, &json!({ "a": [1, 2, 3] })),
            Err(JqlRunnerError::ParsingError(JqlParserError::TruncateError(
                [
                    Token::KeySelector("a"),
                    Token::TruncateOperator,
                    Token::KeySelector("b")
                ]
                .stringify(),
            )))
        );
    }
}
