use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

use crate::tokens::{Index, LensValue};

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

/// A combinator which parses a list of keys surrounded by curly braces.
pub(crate) fn parse_multi_key<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<&'a str>> {
    trim(delimited(char('{'), parse_keys, char('}')))
}

/// A combinator which parses an array of `Index`.
pub(crate) fn parse_array_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('['), parse_indexes, char(']')))
}

/// A combinator which parses an array range.
pub(crate) fn parse_array_range<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)> {
    trim(delimited(
        char('['),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char(']'),
    ))
}

/// A combinator which parses a list of index surrounded by curly braces.
pub(crate) fn parse_object_index<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Index>> {
    trim(delimited(char('{'), parse_indexes, char('}')))
}

/// A combinator which parses an object range.
pub(crate) fn parse_object_range<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, (Option<Index>, Option<Index>)> {
    trim(delimited(
        char('{'),
        separated_pair(opt(parse_number), tag(":"), opt(parse_number)),
        char('}'),
    ))
}

/// A combinator which parses a `LensValue::Null`.
pub(crate) fn parse_null_lens_value<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, LensValue, E>
where
    E: ParseError<&'a str>,
{
    value(LensValue::Null, tag("null"))
}

/// A combinator which parses a `LensValue::String`.
pub(crate) fn parse_string_lens_value<'a, E>()
-> impl FnMut(&'a str) -> IResult<&'a str, LensValue, E>
where
    E: ParseError<&'a str>,
{
    map(parse_key(), LensValue::String)
}

/// A combinator which parses a `LensValue::Number`.
pub(crate) fn parse_number_lens_value(input: &str) -> IResult<&str, LensValue> {
    map_res(recognize(digit1), |index: &str| {
        index.parse::<u32>().map(LensValue::Number)
    })(input)
}

/// A combinator which parses any lens value.
pub(crate) fn parse_lens_value<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, LensValue> {
    alt((
        parse_null_lens_value(),
        parse_string_lens_value(),
        parse_number_lens_value,
    ))
}

/// A combinator which parses a lens.
pub(crate) fn parse_lens<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, (&'a str, Option<LensValue>)> {
    trim(pair(
        parse_key(),
        opt(preceded(trim(tag("=")), parse_lens_value())),
    ))
}

/// A combinator which parses a list of lenses.
pub(crate) fn parse_lenses<'a>()
-> impl FnMut(&'a str) -> IResult<&'a str, Vec<(&'a str, Option<LensValue<'a>>)>> {
    trim(delimited(
        tag("|={"),
        separated_list1(trim(tag(",")), trim(parse_lens())),
        char('}'),
    ))
}

/// A combinator which parses a flatten operator.
pub(crate) fn parse_flatten_operator<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag(".."))
}

/// A combinator which parses a pipe in operator.
pub(crate) fn parse_pipe_in_operator<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("|>"))
}

/// A combinator which parses a pipe out operator.
pub(crate) fn parse_pipe_out_operator<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("<|"))
}

/// A combinator which parses a truncate operator.
pub(crate) fn parse_truncate_operator<'a, E>() -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str>,
{
    trim(tag("!"))
}

#[cfg(test)]
mod tests {
    use nom::{bytes::complete::tag, error::Error};

    use super::{
        parse_array_index, parse_array_range, parse_flatten_operator, parse_indexes, parse_key,
        parse_lens, parse_lenses, parse_multi_key, parse_null_lens_value, parse_number,
        parse_number_lens_value, parse_object_index, parse_object_range, parse_pipe_in_operator,
        parse_pipe_out_operator, parse_string_lens_value, parse_truncate_operator, trim,
    };
    use crate::tokens::{Index, LensValue};

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
        assert_eq!(
            parse_flatten_operator::<Error<_>>()("..").unwrap(),
            ("", "..")
        );
        assert!(parse_flatten_operator::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_pipe_in() {
        assert_eq!(
            parse_pipe_in_operator::<Error<_>>()("|>").unwrap(),
            ("", "|>")
        );
        assert!(parse_pipe_in_operator::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_pipe_out() {
        assert_eq!(
            parse_pipe_out_operator::<Error<_>>()("<|").unwrap(),
            ("", "<|")
        );
        assert!(parse_pipe_out_operator::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_truncate() {
        assert_eq!(
            parse_truncate_operator::<Error<_>>()("!").unwrap(),
            ("", "!")
        );
        assert!(parse_truncate_operator::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_null_lens_value() {
        assert_eq!(
            parse_null_lens_value::<Error<_>>()("null").unwrap(),
            ("", LensValue::Null)
        );
        assert!(parse_null_lens_value::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_string_lens_value() {
        assert_eq!(
            parse_string_lens_value::<Error<_>>()(r#""abc""#).unwrap(),
            ("", LensValue::String("abc"))
        );
        assert!(parse_string_lens_value::<Error<_>>()("").is_err());
    }

    #[test]
    fn check_parse_number_lens_value() {
        assert_eq!(
            parse_number_lens_value("123").unwrap(),
            ("", LensValue::Number(123))
        );
        assert!(parse_number_lens_value("").is_err());
    }

    #[test]
    fn check_parse_lens() {
        assert_eq!(parse_lens()(r#""abc""#).unwrap(), ("", ("abc", None)));
        assert_eq!(
            parse_lens()(r#""abc"=null"#).unwrap(),
            ("", ("abc", Some(LensValue::Null)))
        );
        assert_eq!(
            parse_lens()(r#""abc"="def""#).unwrap(),
            ("", ("abc", Some(LensValue::String("def"))))
        );
        assert_eq!(
            parse_lens()(r#""abc"=123"#).unwrap(),
            ("", ("abc", Some(LensValue::Number(123))))
        );
        assert!(parse_lens()("").is_err());
    }
    #[test]
    fn check_parse_lenses() {
        assert_eq!(
            parse_lenses()(r#"|={"abc","bcd"=123,"efg"=null,"hij"="test"}"#).unwrap(),
            (
                "",
                vec![
                    ("abc", None),
                    ("bcd", Some(LensValue::Number(123))),
                    ("efg", Some(LensValue::Null)),
                    ("hij", Some(LensValue::String("test"))),
                ]
            )
        );
    }
}
