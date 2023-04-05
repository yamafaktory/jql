use thiserror::Error;

/// Error type returned by the parser.
#[derive(Debug, Error, PartialEq)]
pub enum JqlParserError {
    /// Empty input error.
    #[error("No input provided")]
    NoInputProvided,

    /// Unable to parse error.
    #[error("Unable to parse input {unparsed} after {tokens}")]
    UnableToParseInput {
        /// Tokens found while parsing.
        tokens: String,
        /// Unparsed content.
        unparsed: String,
    },

    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,
}
