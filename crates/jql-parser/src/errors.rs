use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum JqlParserError<'a> {
    /// Empty input error.
    #[error("No input provided")]
    NoInputProvided,

    /// General parsing error.
    #[error("Enable to parse input {0}")]
    EnableToParseInput(&'a str),
}
