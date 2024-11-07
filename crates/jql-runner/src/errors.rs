use jql_parser::errors::JqlParserError;
use serde_json::Value;
use thiserror::Error;

static SLICE_SEP: &str = " ... ";
static SLICE_LEN: usize = 7;
static SEP: &str = ", ";

/// Joins multiple `String`.
fn join(values: &[String]) -> String {
    values.join(SEP)
}

/// Shortens a JSON `Value` for error injection.
fn shorten(json: &Value) -> String {
    let full_json_string = json.to_string();

    if full_json_string.len() < SLICE_LEN * 2 + SLICE_SEP.len() {
        return full_json_string;
    }

    let start_slice = &full_json_string[..SLICE_LEN];
    let end_slice = &full_json_string[full_json_string.len() - SLICE_LEN..];

    [start_slice, end_slice].join(SLICE_SEP)
}

/// Returns the type of a JSON `Value`.
fn get_json_type(json: &Value) -> &str {
    match json {
        Value::Array(_) => "array",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
        Value::Number(_) => "number",
        Value::Object(_) => "object",
        Value::String(_) => "string",
    }
}

/// Error type returned by the runner.
#[derive(Debug, Error, PartialEq)]
pub enum JqlRunnerError {
    /// Empty query error.
    #[error("Query is empty")]
    EmptyQueryError,

    /// Flatten error.
    #[error("Value {0} is neither an array nor an object and can't be flattened")]
    FlattenError(Value),

    /// Index out of bounds error.
    #[error("Index {index} in parent {parent} is out of bounds")]
    IndexOutOfBoundsError {
        /// Index.
        index: usize,
        /// Parent value.
        parent: Value,
    },

    /// Invalid array error.
    #[error("Value {} is not a JSON array ({})", .0, get_json_type(.0))]
    InvalidArrayError(Value),

    /// Invalid object error.
    #[error("Value {} is not a JSON object ({})", .0, get_json_type(.0))]
    InvalidObjectError(Value),

    /// Key not found error.
    #[error(r#"Key "{key}" doesn't exist in parent {}"#, shorten(parent))]
    KeyNotFoundError {
        /// Key not found.
        key: String,
        /// Parent value.
        parent: Value,
    },

    /// Keys not found error.
    #[error("Keys {} don't exist in parent {}", join(keys), shorten(parent))]
    MultiKeyNotFoundError {
        /// Keys not found.
        keys: Vec<String>,
        /// Parent value.
        parent: Value,
    },

    /// Parsing error.
    #[error(transparent)]
    ParsingError(#[from] JqlParserError),

    /// Pipe in error.
    #[error("Pipe in operator used on {0} which is not an array")]
    PipeInError(Value),

    /// Pipe in error.
    #[error("Pipe out operator used without a preceding pipe in operator")]
    PipeOutError,

    /// Range out of bounds error.
    #[error("Range [{start}:{end}] in parent {parent} is out of bounds")]
    RangeOutOfBoundsError {
        /// Start range.
        start: usize,
        /// End range.
        end: usize,
        /// parent value.
        parent: Value,
    },

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::{
        get_json_type,
        join,
        shorten,
    };

    #[test]
    fn check_get_json_type() {
        assert_eq!(get_json_type(&json!([])), "array");
        assert_eq!(get_json_type(&json!(true)), "boolean");
        assert_eq!(get_json_type(&json!(null)), "null");
        assert_eq!(get_json_type(&json!(1)), "number");
        assert_eq!(get_json_type(&json!({})), "object");
        assert_eq!(get_json_type(&json!("a")), "string");
    }

    #[test]
    fn check_join() {
        assert_eq!(
            join(&["a".to_string(), "b".to_string(), "c".to_string()]),
            "a, b, c".to_string()
        );
    }

    #[test]
    fn check_shorten() {
        assert_eq!(shorten(&json!("thismakesnosense")), r#""thismakesnosense""#);
        assert_eq!(
            shorten(&json!({ "a": { "b": { "c": [1, 2 ,3, 4, 5, 6, 7, 8, 9] } } })),
            r#"{"a":{" ... 8,9]}}}"#.to_string()
        );
    }
}
