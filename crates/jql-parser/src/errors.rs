use thiserror::Error;

/// Error type returned by the parser.
#[derive(Debug, Error, PartialEq)]
pub enum JqlParserError {
    /// Empty input error.
    #[error("Empty input")]
    EmptyInputError,

    /// Parsing error.
    #[error("Unable to parse input {unparsed} after {tokens}")]
    ParsingError {
        /// Tokens found while parsing.
        tokens: String,
        /// Unparsed content.
        unparsed: String,
    },

    /// Truncate error.
    #[error("Truncate operator found as non last element or multiple times in {0}")]
    TruncateError(String),

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}
