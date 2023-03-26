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
fn trim<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// fn todo(key: &str) -> bool {
//     let mut map = Map::new();
//     let f = map.insert(key.to_string(), Value::Null);
// }

fn parse_number(input: &str) -> IResult<&str, Index> {
    map_res(recognize(digit1), |index: &str| {
        index.parse::<u32>().map(Index)
    })(input)
}

fn parse_indexes(input: &str) -> IResult<&str, Vec<Index>> {
    separated_list1(trim(tag(",")), trim(parse_number))(input)
}

fn parse_keys(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(trim(tag(",")), trim(parse_key()))(input)
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_multi_key<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<&'a str>> {
    trim(delimited(char('{'), parse_keys, char('}')))
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_array_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('['), parse_indexes, char(']')))
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_array_range<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)>
{
    trim(delimited(
        char('['),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char(']'),
    ))
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_key<'a, E: ParseError<&'a str>>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    trim(delimited(char('"'), take_until(r#"""#), char('"')))
}

/// A combinator which parses a filter selector.
fn parse_filter<'a, E: ParseError<&'a str>>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
{
    trim(tag("|"))
}

/// Parses the provided input and map to the matching grammar selector.
fn parse_fragment(input: &str) -> IResult<&str, Grammar> {
    alt((
        value(Grammar::FilterSelector, parse_filter()),
        map(parse_key(), Grammar::KeySelector),
        map(parse_array_index(), Grammar::ArrayIndexSelector),
        map(parse_array_range(), Grammar::ArrayRangeSelector),
        map(parse_multi_key(), Grammar::MultiKeySelector),
    ))(input)
}

/// TODO
///
///
/// # Errors
/// TODO
pub fn parse(input: &str) -> Result<Vec<Grammar>, JqlParserError> {
    let mut parser_iterator = iterator(input, parse_fragment);
    let parsed = parser_iterator.collect::<Vec<Grammar>>();
    let result: IResult<_, _> = parser_iterator.finish();

    match result {
        Ok((rest, _)) => {
            if !rest.is_empty() {
                return Err(JqlParserError::EnableToParseInput(rest));
            }

            Ok(parsed)
        }
        Err(err) => {
            dbg!(err);
            Ok(parsed)
            // return Err(JqlParserError::EnableToParseInput);
        }
    }
}

/// Index used for arrays and objects.
/// Internally uses a `u32` with the newtype pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index(u32);

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Array Index Selector [{}]", self.0)
    }
}

/// Parser grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grammar<'a> {
    /// Array index selector.
    ArrayIndexSelector(Vec<Index>),
    /// Array range selector.
    ArrayRangeSelector((Option<Index>, Option<Index>)),
    /// Filter selector.
    FilterSelector,
    /// Key selector.
    KeySelector(&'a str),
    /// Multi key selector.
    MultiKeySelector(Vec<&'a str>),
}

impl<'a> fmt::Display for Grammar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grammar::ArrayIndexSelector(indexes) => {
                write!(f, "Array Index Selector [{}]", indexes[0])
            }
            Grammar::ArrayRangeSelector(range) => {
                write!(f, "Array Range Selector []")
            }
            _ => write!(f, ""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, parse_fragment, Grammar, Index};

    #[test]
    fn check() {
        // Filter selector.
        assert_eq!(parse_fragment(r#"|"#), Ok(("", Grammar::FilterSelector)));
        assert_eq!(parse_fragment(r#" | "#), Ok(("", Grammar::FilterSelector)));

        // Key selector.
        assert_eq!(
            parse_fragment(r#""one""#),
            Ok(("", Grammar::KeySelector("one")))
        );
        assert_eq!(
            parse_fragment(r#" "one" "#),
            Ok(("", Grammar::KeySelector("one")))
        );

        // Array index selector.
        assert_eq!(
            parse_fragment(r#"[0,1,2]"#),
            Ok((
                "",
                Grammar::ArrayIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
        assert_eq!(
            parse_fragment(r#" [ 0 , 1 , 2 ] "#),
            Ok((
                "",
                Grammar::ArrayIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );

        // Array range selector.
        assert_eq!(
            parse_fragment(r#"[0:2]"#),
            Ok((
                "",
                Grammar::ArrayRangeSelector((Some(Index(0)), Some(Index(2))))
            ))
        );
        assert_eq!(
            parse_fragment(r#"[:2]"#),
            Ok(("", Grammar::ArrayRangeSelector((None, Some(Index(2))))))
        );
        assert_eq!(
            parse_fragment(r#"[0:]"#),
            Ok(("", Grammar::ArrayRangeSelector((Some(Index(0)), None))))
        );
        assert_eq!(
            parse_fragment(r#"[:]"#),
            Ok(("", Grammar::ArrayRangeSelector((None, None))))
        );

        // Multi key selector.
        assert_eq!(
            parse_fragment(r#"{"one","two","three"}"#),
            Ok(("", Grammar::MultiKeySelector(vec!["one", "two", "three"])))
        );
        assert_eq!(
            parse_fragment(r#" { "one", "two" , "three" } "#),
            Ok(("", Grammar::MultiKeySelector(vec!["one", "two", "three"])))
        );

        // Full parser.
        assert_eq!(
            parse(r#"[9,0]sdf"#),
            Ok(vec![Grammar::KeySelector("country")])
        );
    }
}
