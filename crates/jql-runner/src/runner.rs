use jql_parser::{
    errors::JqlParserError,
    group::split,
    parser::parse,
    tokens::Token,
};
use rayon::prelude::*;
use serde_json::{
    json,
    Value,
};

use crate::errors::JqlRunnerError;

/// Takes a raw input to parse and a JSON `Value`.
pub fn runner(input: &str, json: &Value) -> Result<Value, JqlRunnerError> {
    if input.is_empty() {
        return Err(JqlRunnerError::NoInputProvided);
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

    result.map(|v| json!(v))
}

fn group_runner(tokens: &Vec<&Token>, json: &Value) -> Result<Value, JqlRunnerError> {
    let result: Result<Value, JqlRunnerError> = tokens
        .par_iter()
        .try_fold_with(json!(null), |mut acc: Value, &token| match token {
            Token::ArrayIndexSelector(_) => todo!(),
            Token::ArrayRangeSelector(_) => todo!(),
            Token::FlattenOperator => todo!(),
            Token::GroupSeparator => todo!(),
            Token::KeySelector(key) => {
                if let Some(value) = json.get(key) {
                    Ok(value.clone())
                } else {
                    Err(JqlRunnerError::UnknownError)
                }
            }
            Token::LensSelector(_) => todo!(),
            Token::MultiKeySelector(_) => todo!(),
            Token::ObjectIndexSelector(_) => todo!(),
            Token::ObjectRangeSelector(_) => todo!(),
            Token::PipeInOperator => todo!(),
            Token::PipeOutOperator => todo!(),
            Token::TruncateOperator => todo!(),
        })
        .try_reduce(|| json!(null), |mut a, b| Ok(b));

    result
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

    use super::runner;
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_runner() {
        assert_eq!(runner("", &json!("")), Err(JqlRunnerError::NoInputProvided));
        assert_eq!(
            runner(r#""a"b"#, &json!({ "a": 1 })),
            Err(JqlRunnerError::Parsing(
                JqlParserError::UnableToParseInput {
                    tokens: [Token::KeySelector("a")].stringify(),
                    unparsed: "b".to_string(),
                }
            ))
        );
        assert_eq!(
            runner(r#""a","b""#, &json!({ "a": 1, "b": 2 })),
            Ok(json!([1, 2]))
        );
    }
}
