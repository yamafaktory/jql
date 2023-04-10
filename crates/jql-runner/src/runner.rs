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
        get_array_range,
        get_flattened_array,
    },
    errors::JqlRunnerError,
    object::{
        get_flattened_object,
        get_object_indexes,
        get_object_key,
        get_object_multi_key,
    },
};

/// Takes a raw input as a slice string to parse and a reference of a JSON
/// `Value`.
/// Returns a JSON `Value` or an error.
pub fn raw_runner(input: &str, json: &Value) -> Result<Value, JqlRunnerError> {
    if input.is_empty() {
        return Err(JqlRunnerError::EmptyInputError);
    }

    let tokens = parse(input)?;

    token_runner(tokens, json)
}

/// Takes a vector of `Tokens` to parse and a reference of a JSON
/// `Value`.
/// Returns a JSON `Value` or an error.
pub fn token_runner(tokens: Vec<Token>, json: &Value) -> Result<Value, JqlRunnerError> {
    let groups = split(&tokens);

    let result = groups
        .par_iter()
        .try_fold_with(vec![], |mut acc: Vec<Value>, group| {
            acc.push(group_runner(group, json)?);

            Ok::<Vec<Value>, JqlRunnerError>(acc)
        })
        .try_reduce(
            || vec![],
            |mut a, b| {
                a.extend(b);

                Ok(a)
            },
        );

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
        .try_fold(json.clone(), |mut acc: Value, &token| match token {
            Token::ArrayIndexSelector(indexes) => get_array_indexes(indexes, &acc),
            Token::ArrayRangeSelector(range) => get_array_range(range, &mut acc),
            Token::FlattenOperator => match acc {
                Value::Array(_) => get_flattened_array(&acc),
                Value::Object(_) => get_flattened_object(&acc),
                _ => Err(JqlRunnerError::FlattenError(acc)),
            },
            Token::GroupSeparator => unreachable!(),
            Token::KeySelector(key) => get_object_key(key, &acc),
            Token::LensSelector(_) => todo!(),
            Token::MultiKeySelector(keys) => get_object_multi_key(keys, &mut acc),
            Token::ObjectIndexSelector(indexes) => get_object_indexes(indexes, &mut acc),
            Token::ObjectRangeSelector(_) => todo!(),
            Token::PipeInOperator => todo!(),
            Token::PipeOutOperator => todo!(),
            Token::TruncateOperator => todo!(),
        })
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

    use super::raw_runner;
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_runner_empty_input_error() {
        assert_eq!(
            raw_runner("", &json!("")),
            Err(JqlRunnerError::EmptyInputError)
        );
    }

    #[test]
    fn check_runner_parsing_error() {
        assert_eq!(
            raw_runner(r#""a"b"#, &json!({ "a": 1 })),
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
            raw_runner(r#""b""#, &parent),
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
            raw_runner("[1]", &parent),
            Err(JqlRunnerError::IndexOutOfBoundsError { index: 1, parent })
        );
    }

    #[test]
    fn check_runner_success() {
        assert_eq!(
            raw_runner(r#""a","b""#, &json!({ "a": 1, "b": 2 })),
            Ok(json!([1, 2]))
        );
        assert_eq!(
            raw_runner(r#""a""b""#, &json!({ "a": { "b": 2 } })),
            Ok(json!(2))
        );
        assert_eq!(
            raw_runner("[4,2,0]", &json!(["a", "b", "c", "d", "e"])),
            Ok(json!(["e", "c", "a"]))
        );
    }
}
