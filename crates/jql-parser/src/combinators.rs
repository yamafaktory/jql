use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, opt, recognize},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::tokens::Index;

/// A combinator which takes an `inner` parser and produces a parser which also
/// consumes both leading and trailing whitespaces, returning the output of
/// `inner`.
pub(crate) fn trim<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator which parses a stringified number as an `Index`.
pub(crate) fn parse_number(input: &str) -> IResult<&str, Index> {
    map_res(recognize(digit1), |index: &str| {
        index.parse::<u32>().map(Index)
    })(input)
}

/// A combinator which parses a key surrounded by double quotes.
pub(crate) fn parse_key<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(delimited(char('"'), take_until(r#"""#), char('"')))
}

/// A combinator which parses a list of `Index`.
pub(crate) fn parse_indexes(input: &str) -> IResult<&str, Vec<Index>> {
    separated_list1(trim(tag(",")), trim(parse_number))(input)
}

/// A combinator which parses a list of keys.
fn parse_keys(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(trim(tag(",")), trim(parse_key()))(input)
}

/// A combinator which parses a key surrounded by double quotes.
pub(crate) fn parse_multi_key<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<&'a str>> {
    trim(delimited(char('{'), parse_keys, char('}')))
}

/// A combinator which parses an array of `Index`.
pub(crate) fn parse_array_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('['), parse_indexes, char(']')))
}

/// A combinator which parses an array of `Range`.
pub(crate) fn parse_array_range<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)> {
    trim(delimited(
        char('['),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char(']'),
    ))
}

/// A combinator which parses a key surrounded by double quotes.
pub(crate) fn parse_object_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('{'), parse_indexes, char('}')))
}

/// A combinator which parses a key surrounded by double quotes.
pub(crate) fn parse_object_range<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)> {
    trim(delimited(
        char('{'),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char('}'),
    ))
}

/// A combinator which parses a flatten operator.
pub(crate) fn parse_flatten<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag(".."))
}

/// A combinator which parses a pipe in operator.
pub(crate) fn parse_pipe_in<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("|>"))
}

/// A combinator which parses a pipe out operator.
pub(crate) fn parse_pipe_out<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("<|"))
}

/// A combinator which parses a truncate operator.
pub(crate) fn parse_truncate<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("!"))
}

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::tag, error::Error};

    use super::{
        parse_array_index, parse_array_range, parse_flatten, parse_indexes, parse_key,
        parse_multi_key, parse_number, parse_object_index, parse_object_range, parse_pipe_in,
        parse_pipe_out, parse_truncate, trim,
    };
    use crate::tokens::Index;

    #[test]
    fn check_trim() {
        assert_eq!(
            trim(tag::<&str, &str, Error<_>>("abc"))(" abc ").unwrap(),
            ("", "abc")
        );
    }

    #[test]
    fn check_parse_number() {
        assert_eq!(parse_number("123").unwrap(), ("", Index(123)));
        assert!(parse_number("abc").is_err());
        assert!(parse_number("abc123").is_err());
    }

    #[test]
    fn check_parse_key() {
        assert_eq!(parse_key::<Error<_>>()(r#""abc""#).unwrap(), ("", "abc"));
        assert!(parse_key::<Error<_>>()("abc").is_err());
    }

    #[test]
    fn check_parse_indexes() {
        assert_eq!(parse_indexes("123").unwrap(), ("", vec![Index(123)]));
        assert_eq!(
            parse_indexes("123,456,789").unwrap(),
            ("", vec![Index(123), Index(456), Index(789)])
        );
        assert!(parse_indexes("abc").is_err());
    }

    #[test]
    fn check_parse_multi_key() {
        assert_eq!(parse_multi_key()(r#"{"abc"}"#).unwrap(), ("", vec!["abc"]));
        assert_eq!(
            parse_multi_key()(r#"{"abc","def"}"#).unwrap(),
            ("", vec!["abc", "def"])
        );
        assert!(parse_multi_key()("{}").is_err());
        assert!(parse_multi_key()("{123}").is_err());
    }

    #[test]
    fn check_parse_array_index() {
        assert_eq!(parse_array_index()("[1]").unwrap(), ("", vec![Index(1)]));
        assert_eq!(
            parse_array_index()("[1,2,3]").unwrap(),
            ("", vec![Index(1), Index(2), Index(3)])
        );
        assert!(parse_array_index()("[]").is_err());
        assert!(parse_array_index()(r#"["1"]"#).is_err());
    }

    #[test]
    fn check_parse_array_range() {
        assert_eq!(parse_array_range()("[:]").unwrap(), ("", (None, None)));
        assert_eq!(
            parse_array_range()("[1:]").unwrap(),
            ("", (Some(Index(1)), None))
        );
        assert_eq!(
            parse_array_range()("[:1]").unwrap(),
            ("", (None, Some(Index(1))))
        );
        assert_eq!(
            parse_array_range()("[1:3]").unwrap(),
            ("", (Some(Index(1)), Some(Index(3))))
        );
        assert!(parse_array_range()("[]").is_err());
    }

    #[test]
    fn check_parse_object_index() {
        assert_eq!(parse_object_index()("{1}").unwrap(), ("", vec![Index(1)]));
        assert_eq!(
            parse_object_index()("{1,2}").unwrap(),
            ("", vec![Index(1), Index(2)])
        );
        assert!(parse_object_index()("{}").is_err());
    }

    #[test]
    fn check_parse_object_range() {
        assert_eq!(parse_object_range()("{:}").unwrap(), ("", (None, None)));
        assert_eq!(
            parse_object_range()("{1:}").unwrap(),
            ("", (Some(Index(1)), None))
        );
        assert_eq!(
            parse_object_range()("{:1}").unwrap(),
            ("", (None, Some(Index(1))))
        );
        assert_eq!(
            parse_object_range()("{1:3}").unwrap(),
            ("", (Some(Index(1)), Some(Index(3))))
        );
        assert!(parse_object_range()("{}").is_err());
    }

    #[test]
    fn check_parse_flatten() {
        assert_eq!(parse_flatten::<Error<_>>()("..").unwrap(), ("", ".."));
        assert!(parse_flatten::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_pipe_in() {
        assert_eq!(parse_pipe_in::<Error<_>>()("|>").unwrap(), ("", "|>"));
        assert!(parse_pipe_in::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_pipe_out() {
        assert_eq!(parse_pipe_out::<Error<_>>()("<|").unwrap(), ("", "<|"));
        assert!(parse_pipe_out::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_truncate() {
        assert_eq!(parse_truncate::<Error<_>>()("!").unwrap(), ("", "!"));
        assert!(parse_truncate::<Error<_>>()("").is_err());
    }
}
