use jql_parser::errors::JqlParserError;
use thiserror::Error;

/// Error type returned by the runner.
#[derive(Debug, Error, PartialEq)]
pub enum JqlRunnerError {
    /// Empty input error.
    #[error("Input is empty")]
    EmptyInputError,

    /// Index not found error.
    #[error("Index {index} doesn't exist on parent {parent}")]
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

    /// Parsing error.
    #[error("Parsing failed")]
    ParsingError(#[from] JqlParserError),

    /// Range out of bounds error.
    #[error("Range [{start}:{end}]  on parent {parent} is out of bounds")]
    RangeOutOfBoundsError {
        /// Start range.
        start: usize,
        /// End range.
        end: usize,
        /// parent node.
        parent: String,
    },

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}
