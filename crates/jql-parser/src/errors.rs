use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum JqlParserError<'a> {
    /// Unknown error.
    #[error("Unknown error")]
    UnknownError,

    /// Empty input error.
    #[error("No input provided")]
    NoInputProvided,

    /// Unable to parse error.
    #[error("Unable to parse input {rest} after {tokens}")]
    UnableToParseInput { tokens: String, rest: &'a str },
}
