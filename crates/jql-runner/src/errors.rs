use jql_parser::errors::JqlParserError;
use serde_json::Value;
use thiserror::Error;

static SLICE_SEP: &str = " ... ";
static SLICE_LEN: usize = 7;
static SEP: &str = ", ";

/// Joins multiple `String`.
fn join(values: &Vec<String>) -> String {
    values.join(SEP)
}

/// Shortens a JSON `Value` for error injection.
fn shorten(json: &Value) -> String {
    let full_json_string = json.to_string();
    let start_slice = &full_json_string[..SLICE_LEN];
    let end_slice = &full_json_string[full_json_string.len() - SLICE_LEN..];

    [start_slice, end_slice].join(SLICE_SEP)
}

/// Error type returned by the runner.
#[derive(Debug, Error, PartialEq)]
pub enum JqlRunnerError {
    /// Empty input error.
    #[error("Input is empty")]
    EmptyInputError,

    /// Index not found error.
    #[error("Index {index} doesn't exist in parent {parent}")]
    IndexNotFoundError {
        /// Index not found.
        index: usize,
        /// Parent node.
        parent: Value,
    },

    /// Invalid array error.
    #[error("Node {0} is not a JSON array")]
    InvalidArrayError(Value),

    /// Invalid object error.
    #[error("Node {0} is not a JSON object")]
    InvalidObjectError(Value),

    /// Key not found error.
    #[error("Key {key} doesn't exist in parent {}", shorten(parent))]
    KeyNotFoundError {
        /// Key not found.
        key: String,
        /// Parent node.
        parent: Value,
    },

    /// Keys not found error.
    #[error("Keys {} don't exist in parent {}", join(keys), shorten(parent))]
    MultiKeyNotFoundError {
        /// Keys not found.
        keys: Vec<String>,
        /// Parent node.
        parent: Value,
    },

    /// Parsing error.
    #[error(transparent)]
    ParsingError(#[from] JqlParserError),

    /// Range out of bounds error.
    #[error("Range [{start}:{end}] in parent {parent} is out of bounds")]
    RangeOutOfBoundsError {
        /// Start range.
        start: usize,
        /// End range.
        end: usize,
        /// parent node.
        parent: Value,
    },

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}

#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::shorten;

    #[test]
    fn check_shorten() {
        let value = json!({ "a": { "b": { "c": [1, 2 ,3, 4, 5, 6, 7, 8, 9] } } });

        assert_eq!(shorten(&value), r#"{"a":{" ... 8,9]}}}"#.to_string());
    }
}
