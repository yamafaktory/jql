use thiserror::Error;

fn display_content(content: &str) -> String {
    if content.is_empty() {
        String::new()
    } else {
        format!(" after {content}")
    }
}

/// Error type returned by the parser.
///
/// Marked `#[non_exhaustive]`: new variants are added as the parser grows, and
/// downstream code must not break when one appears.
#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum JqlParserError {
    /// Empty input error.
    #[error("Empty input")]
    EmptyInputError,

    /// Parsing error.
    #[error("Unable to parse input {unparsed}{}", display_content(tokens))]
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

#[cfg(test)]
mod tests {

    use super::display_content;

    #[test]
    fn check_display_content() {
        assert_eq!(display_content("some"), " after some");
        assert_eq!(display_content(""), "");
    }
}
