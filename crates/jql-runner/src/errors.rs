use jql_parser::errors::JqlParserError;
use thiserror::Error;

/// Error type returned by the runner.
#[derive(Debug, Error, PartialEq)]
pub enum JqlRunnerError {
    /// Empty input error.
    #[error("No input provided")]
    NoInputProvidedError,

    /// Parsing error.
    #[error("Parsing failed")]
    ParsingError(#[from] JqlParserError),

    /// Index not found error.
    #[error("index {index} doesn't exist on parent {parent}")]
    IndexNotFoundError {
        /// Index not found.
        index: usize,
        /// Parent node.
        parent: String,
    },

    /// Key not found error.
    #[error("Key {key} doesn't exist on parent {parent}")]
    KeyNotFoundError {
        /// Key not found.
        key: String,
        /// Parent node.
        parent: String,
    },

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}
