use jql_parser::errors::JqlParserError;
use thiserror::Error;

/// Error type returned by the runner.
#[derive(Debug, Error, PartialEq)]
pub enum JqlRunnerError {
    /// Empty input error.
    #[error("No input provided")]
    NoInputProvided,

    /// Parsing error.
    #[error("Parsing failed")]
    Parsing(#[from] JqlParserError),

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}
