#![deny(clippy::pedantic)]
#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! TODO
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        complete::{digit1, multispace0},
        streaming::char,
    },
    combinator::{map, map_res, opt, recognize, value},
    error::{FromExternalError, ParseError},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

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

fn my_u64<'a>(input: &'a str) -> IResult<&'a str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_indexes(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), my_u64)(input)
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_array_index<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    trim(delimited(char('['), parse_indexes(input), char(']')))
}

/// A combinator which parses a key surrounded by double quotes.
fn parse_key<'a, E: ParseError<&'a str>>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    trim(delimited(char('"'), take_until(r#"""#), char('"')))
}

/// A combinator which parses a dot selector.
fn parse_dot<'a, E: ParseError<&'a str>>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E> {
    trim(tag("."))
}

/// A combinator which parses a filter selector.
fn parse_filter<'a, E: ParseError<&'a str>>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
{
    trim(tag("|"))
}

/// Parses the provided input and map to the matching grammar selector.
fn parse<'a, E>(input: &'a str) -> IResult<&'a str, Grammar<'a>, E>
where
    E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
{
    alt((
        value(Grammar::DotSelector, parse_dot()),
        value(Grammar::FilterSelector, parse_filter()),
        map(parse_key(), Grammar::KeySelector),
    ))(input)
}

/// TODO
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grammar<'a> {
    /// Array index selector.
    ArrayIndexSelector(&'a [usize]),
    /// Array range selector.
    ArrayRangeSelector((Option<usize>, Option<usize>)),
    /// Dot selector.
    DotSelector,
    /// Filter selector.
    FilterSelector,
    /// Key selector.
    KeySelector(&'a str),
}

// TODO
// https://docs.rs/nom/latest/nom/trait.Finish.html#tymethod.finish

#[cfg(test)]
mod tests {
    use super::{parse, Grammar};

    #[test]
    fn check() {
        // Dot selector.
        assert_eq!(parse::<()>(r#"."#), Ok(("", Grammar::DotSelector)));
        assert_eq!(parse::<()>(r#" . "#), Ok(("", Grammar::DotSelector)));

        // Filter selector.
        assert_eq!(parse::<()>(r#"|"#), Ok(("", Grammar::FilterSelector)));
        assert_eq!(parse::<()>(r#" | "#), Ok(("", Grammar::FilterSelector)));

        // Key selector.
        assert_eq!(
            parse::<()>(r#""one""#),
            Ok(("", Grammar::KeySelector("one")))
        );
        assert_eq!(
            parse::<()>(r#" "one" "#),
            Ok(("", Grammar::KeySelector("one")))
        );
    }
}
