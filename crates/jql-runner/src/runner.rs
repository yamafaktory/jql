use jql_parser::parser;
use serde_json::Value;

use crate::errors::JqlRunnerError;

fn runner(input: &str, json: &Value) -> Result<(), JqlRunnerError> {
    if input.is_empty() {
        return Err(JqlRunnerError::NoInputProvided);
    }

    let parsed = parser::parse(input);

    dbg!(parsed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::runner;
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_runner() {
        assert_eq!(runner("", &json!("")), Err(JqlRunnerError::NoInputProvided));
        assert_eq!(
            runner(r#""a""#, &json!({ "a": 1 })),
            Err(JqlRunnerError::UnknownError)
        );
    }
}
