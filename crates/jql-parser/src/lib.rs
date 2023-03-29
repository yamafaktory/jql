#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! TODO
use std::fmt;

use errors::JqlParserError;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, multispace0},
    combinator::{iterator, map, map_res, opt, recognize, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

mod errors;

/// A combinator which takes an `inner` parser and produces a parser which also
/// consumes both leading and trailing whitespace, returning the output of
/// `inner`.
fn trim<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator which parses a stringified number as an `Index`.
fn parse_number(input: &str) -> IResult<&str, Index> {
    map_res(recognize(digit1), |index: &str| {
        index.parse::<u32>().map(Index)
    })(input)
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_key<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(delimited(char('"'), take_until(r#"""#), char('"')))
}

/// A combinator which parses a list of `Index`.
fn parse_indexes(input: &str) -> IResult<&str, Vec<Index>> {
    separated_list1(trim(tag(",")), trim(parse_number))(input)
}

/// A combinator which parses a list of keys.
fn parse_keys(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(trim(tag(",")), trim(parse_key()))(input)
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_multi_key<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<&'a str>> {
    trim(delimited(char('{'), parse_keys, char('}')))
}

/// A combinator which parses an array of `Index`.
fn parse_array_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('['), parse_indexes, char(']')))
}

/// A combinator which parses an array of `Range`.
fn parse_array_range<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)>
{
    trim(delimited(
        char('['),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char(']'),
    ))
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_object_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('{'), parse_indexes, char('}')))
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_object_range<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)> {
    trim(delimited(
        char('{'),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char('}'),
    ))
}

/// A combinator which parses a flatten operator.
fn parse_flatten<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag(".."))
}

/// A combinator which parses a pipe in operator.
fn parse_pipe_in<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("|>"))
}

/// A combinator which parses a pipe out operator.
fn parse_pipe_out<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("<|"))
}

/// A combinator which parses a truncate operator.
fn parse_truncate<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("!"))
}

fn tokens_to_string(tokens: &[Token]) -> String {
    tokens
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join(", ")
}

/// Parses the provided input and map it to the first matching token.
fn parse_fragment(input: &str) -> IResult<&str, Token> {
    alt((
        map(parse_array_index(), Token::ArrayIndexSelector),
        map(parse_array_range(), |(start, end)| {
            Token::ArrayRangeSelector(Range(start, end))
        }),
        map(parse_key(), Token::KeySelector),
        map(parse_multi_key(), Token::MultiKeySelector),
        map(parse_object_index(), Token::ObjectIndexSelector),
        map(parse_object_range(), |(start, end)| {
            Token::ObjectRangeSelector(Range(start, end))
        }),
        value(Token::FlattenOperator, parse_flatten()),
        value(Token::PipeInOperator, parse_pipe_in()),
        value(Token::PipeOutOperator, parse_pipe_out()),
        value(Token::TruncateOperator, parse_truncate()),
    ))(input)
}

/// Parses the provided input and returns a vector of tokens.
///
/// # Errors
///
/// Returns a `JqlParserError` on failure.
pub fn parse(input: &str) -> Result<Vec<Token>, JqlParserError> {
    let mut parser_iterator = iterator(input, parse_fragment);
    let parsed = parser_iterator.collect::<Vec<Token>>();
    let result: IResult<_, _> = parser_iterator.finish();

    match result {
        Ok((rest, _)) => {
            if !rest.is_empty() {
                return Err(JqlParserError::UnableToParseInput {
                    rest,
                    tokens: tokens_to_string(&parsed),
                });
            }

            Ok(parsed)
        }
        Err(_) => Err(JqlParserError::UnknownError),
    }
}

/// `Index` used for arrays and objects.
/// Internally mapped to a `u32` with the newtype pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index(u32);

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index ({})", self.0)
    }
}

/// `Range` used for arrays and objects.
/// Internally mapped to a tuple of `Option` of `Index`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range(Option<Index>, Option<Index>);

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let format_bound = |bound: &Option<Index>| match bound {
            Some(index) => index.to_string(),
            None => String::new(),
        };

        write!(
            f,
            "Range [{}:{}]",
            format_bound(&self.0),
            format_bound(&self.1)
        )
    }
}

/// Parser tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    /// Array index selector.
    ArrayIndexSelector(Vec<Index>),
    /// Array range selector.
    ArrayRangeSelector(Range),
    /// Flatten operator.
    FlattenOperator,
    /// Key selector.
    KeySelector(&'a str),
    /// Multi key selector.
    MultiKeySelector(Vec<&'a str>),
    /// Object index selector.
    ObjectIndexSelector(Vec<Index>),
    /// Object range selector.
    ObjectRangeSelector(Range),
    /// Pipe in operator.
    PipeInOperator,
    /// Pipe out operator.
    PipeOutOperator,
    /// Truncate operator.
    TruncateOperator,
}

impl<'a> Token<'a> {
    fn get_name(&self) -> &'a str {
        match self {
            Token::ArrayIndexSelector(_) => "Array Index Selector",
            Token::ArrayRangeSelector(_) => "Array Range Selector",
            Token::FlattenOperator => "Flatten Operator",
            Token::KeySelector(_) => "Key Selector",
            Token::MultiKeySelector(_) => "Multi Key Selector",
            Token::ObjectIndexSelector(_) => "Object Index Selector",
            Token::ObjectRangeSelector(_) => "Object Range Selector",
            Token::PipeInOperator => "Pipe In Operator",
            Token::PipeOutOperator => "Pipe Out Operator",
            Token::TruncateOperator => "Truncate Operator",
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::ArrayIndexSelector(indexes) | Token::ObjectIndexSelector(indexes) => {
                let formatted_indexes = indexes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");

                write!(f, "{} [{formatted_indexes}]", self.get_name())
            }
            Token::ArrayRangeSelector(range) | Token::ObjectRangeSelector(range) => {
                write!(f, "{} {}", self.get_name(), range)
            }
            Token::KeySelector(key) => {
                write!(f, "{} {key}", self.get_name())
            }
            Token::MultiKeySelector(multi_key) => {
                let formatted_keys = multi_key.join(", ");

                write!(f, "{} {formatted_keys}", self.get_name())
            }
            Token::FlattenOperator
            | Token::PipeInOperator
            | Token::PipeOutOperator
            | Token::TruncateOperator => {
                write!(f, "{}", self.get_name())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, parse_fragment, tokens_to_string, Index, JqlParserError, Range, Token};

    #[test]
    fn check_array_index_selector() {
        assert_eq!(
            parse_fragment(r#"[0,1,2]"#),
            Ok((
                "",
                Token::ArrayIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
        assert_eq!(
            parse_fragment(r#" [ 0 , 1 , 2 ] "#),
            Ok((
                "",
                Token::ArrayIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
    }

    #[test]
    fn check_array_range_selector() {
        assert_eq!(
            parse_fragment(r#"[0:2]"#),
            Ok((
                "",
                Token::ArrayRangeSelector(Range(Some(Index(0)), Some(Index(2))))
            ))
        );
        assert_eq!(
            parse_fragment(r#"[:2]"#),
            Ok(("", Token::ArrayRangeSelector(Range(None, Some(Index(2))))))
        );
        assert_eq!(
            parse_fragment(r#"[0:]"#),
            Ok(("", Token::ArrayRangeSelector(Range(Some(Index(0)), None))))
        );
        assert_eq!(
            parse_fragment(r#"[:]"#),
            Ok(("", Token::ArrayRangeSelector(Range(None, None))))
        );
    }

    #[test]
    fn check_key_selector() {
        assert_eq!(
            parse_fragment(r#""one""#),
            Ok(("", Token::KeySelector("one")))
        );
        assert_eq!(
            parse_fragment(r#" "one" "#),
            Ok(("", Token::KeySelector("one")))
        );
    }

    #[test]
    fn check_multi_key_selector() {
        assert_eq!(
            parse_fragment(r#"{"one","two","three"}"#),
            Ok(("", Token::MultiKeySelector(vec!["one", "two", "three"])))
        );
        assert_eq!(
            parse_fragment(r#" { "one", "two" , "three" } "#),
            Ok(("", Token::MultiKeySelector(vec!["one", "two", "three"])))
        );
    }

    #[test]
    fn check_object_index_selector() {
        assert_eq!(
            parse_fragment(r#"{0,1,2}"#),
            Ok((
                "",
                Token::ObjectIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
        assert_eq!(
            parse_fragment(r#" { 0 , 1 , 2 } "#),
            Ok((
                "",
                Token::ObjectIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
    }

    #[test]
    fn check_object_range_selector() {
        assert_eq!(
            parse_fragment(r#"{0:2}"#),
            Ok((
                "",
                Token::ObjectRangeSelector(Range(Some(Index(0)), Some(Index(2))))
            ))
        );
        assert_eq!(
            parse_fragment(r#"{:2}"#),
            Ok(("", Token::ObjectRangeSelector(Range(None, Some(Index(2))))))
        );
        assert_eq!(
            parse_fragment(r#"{0:}"#),
            Ok(("", Token::ObjectRangeSelector(Range(Some(Index(0)), None))))
        );
        assert_eq!(
            parse_fragment(r#"{:}"#),
            Ok(("", Token::ObjectRangeSelector(Range(None, None))))
        );
    }

    #[test]
    fn check_flatten_operator() {
        assert_eq!(parse_fragment(r#".."#), Ok(("", Token::FlattenOperator)));
        assert_eq!(parse_fragment(r#" .. "#), Ok(("", Token::FlattenOperator)));
    }

    #[test]
    fn check_pipe_in_operator() {
        assert_eq!(parse_fragment(r#"|>"#), Ok(("", Token::PipeInOperator)));
        assert_eq!(parse_fragment(r#" |> "#), Ok(("", Token::PipeInOperator)));
    }

    #[test]
    fn check_pipe_out_operator() {
        assert_eq!(parse_fragment(r#"<|"#), Ok(("", Token::PipeOutOperator)));
        assert_eq!(parse_fragment(r#" <| "#), Ok(("", Token::PipeOutOperator)));
    }

    #[test]
    fn check_truncate_operator() {
        assert_eq!(parse_fragment(r#"!"#), Ok(("", Token::TruncateOperator)));
        assert_eq!(parse_fragment(r#" ! "#), Ok(("", Token::TruncateOperator)));
    }

    #[test]
    fn check_full_parser() {
        assert_eq!(
            parse(r#""this"[9,0]"#),
            Ok(vec![
                Token::KeySelector("this"),
                Token::ArrayIndexSelector(vec![Index(9), Index(0)])
            ]),
        );
        assert_eq!(
            parse(r#"[9,0]nope"#),
            Err(JqlParserError::UnableToParseInput {
                rest: "nope",
                tokens: tokens_to_string(&[Token::ArrayIndexSelector(vec![Index(9), Index(0)])]),
            })
        );
        assert_eq!(
            parse(r#".."this"[9,0]|>"some"<|"ok"!"#),
            Ok(vec![
                Token::FlattenOperator,
                Token::KeySelector("this"),
                Token::ArrayIndexSelector(vec![Index(9), Index(0)]),
                Token::PipeInOperator,
                Token::KeySelector("some"),
                Token::PipeOutOperator,
                Token::KeySelector("ok"),
                Token::TruncateOperator
            ]),
        );
    }
}
