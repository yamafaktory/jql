use jql_parser::{group::split, parser::parse, tokens::Token};
use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    errors::JqlRunnerError,
    json::{get_array_indexes, get_flattened, get_key, get_multi_key, get_range},
};

/// Takes a raw input as a slice string to parse and a reference of a JSON
/// `Value`.
/// Returns a JSON `Value` or an error.
pub fn runner(input: &str, json: &Value) -> Result<Value, JqlRunnerError> {
    if input.is_empty() {
        return Err(JqlRunnerError::EmptyInputError);
    }

    let parsed = parse(input)?;
    let groups = split(&parsed);

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
/// it has been filtered out by the `runner` function.
fn group_runner(tokens: &[&Token], json: &Value) -> Result<Value, JqlRunnerError> {
    tokens
        .iter()
        .try_fold(json.clone(), |mut acc: Value, &token| match token {
            Token::ArrayIndexSelector(indexes) => get_array_indexes(indexes, &acc),
            Token::ArrayRangeSelector(range) => get_range(range, &mut acc),
            Token::FlattenOperator => get_flattened(&mut acc),
            Token::GroupSeparator => unreachable!(),
            Token::KeySelector(key) => get_key(key, &acc),
            Token::LensSelector(_) => todo!(),
            Token::MultiKeySelector(keys) => get_multi_key(keys, &mut acc),
            Token::ObjectIndexSelector(_) => todo!(),
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
        tokens::{Token, View},
    };
    use serde_json::json;

    use super::runner;
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_runner_empty_input_error() {
        assert_eq!(runner("", &json!("")), Err(JqlRunnerError::EmptyInputError));
    }

    #[test]
    fn check_runner_parsing_error() {
        assert_eq!(
            runner(r#""a"b"#, &json!({ "a": 1 })),
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
            runner(r#""b""#, &parent),
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
            runner("[1]", &parent),
            Err(JqlRunnerError::IndexNotFoundError { index: 1, parent })
        );
    }

    #[test]
    fn check_runner_success() {
        assert_eq!(
            runner(r#""a","b""#, &json!({ "a": 1, "b": 2 })),
            Ok(json!([1, 2]))
        );
        assert_eq!(
            runner(r#""a""b""#, &json!({ "a": { "b": 2 } })),
            Ok(json!(2))
        );
        assert_eq!(
            runner("[4,2,0]", &json!(["a", "b", "c", "d", "e"])),
            Ok(json!(["e", "c", "a"]))
        );
    }
}