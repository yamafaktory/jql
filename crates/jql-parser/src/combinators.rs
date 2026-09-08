use std::{
    borrow::Cow,
    str::Chars,
};

use winnow::{
    Parser,
    Result,
    ascii::{
        digit1,
        multispace0,
        take_escaped,
    },
    combinator::{
        alt,
        delimited,
        dispatch,
        empty,
        fail,
        opt,
        peek,
        preceded,
        repeat,
        separated,
        separated_pair,
    },
    error::ParserError,
    stream::AsChar,
    token::{
        any,
        literal,
        take_till,
        take_while,
    },
};

use crate::tokens::{
    Index,
    LensValue,
    Range,
    Token,
};

/// Colon.
static COLON: char = ':';
/// Comma.
static COMMA: char = ',';
/// Curly brace open.
static CURLY_BRACKET_OPEN: char = '{';
/// Curly brace close.
static CURLY_BRACKET_CLOSE: char = '}';
/// Double-quote.
static DOUBLE_QUOTE: char = '"';
/// Equal.
static EQUAL: char = '=';
/// Square brace open.
static SQUARE_BRACKET_OPEN: char = '[';
/// Square brace close.
static SQUARE_BRACKET_CLOSE: char = ']';

/// False.
static FALSE: &str = "false";
/// True.
static TRUE: &str = "true";

/// Lenses start.
static LENSES_START: &str = "|={";

/// Keys operator.
static KEYS: &str = "@";
/// Flatten operator.
static FLATTEN: &str = "..";
/// Group separator.
static GROUP_SEP: &str = ",";
/// Pipe in operator.
static PIPE_IN: &str = "|>";
/// Pipe out operator.
static PIPE_OUT: &str = "<|";
/// Truncate operator.
static TRUNCATE: &str = "!";

/// A combinator which takes an `inner` parser and produces a parser which also
/// consumes both leading and trailing whitespaces, returning the output of
/// `inner`.
pub(crate) fn trim<'a, F, O, E>(inner: F) -> impl Parser<&'a str, O, E>
where
    E: ParserError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator which parses a stringified number as an `Index`.
pub(crate) fn parse_number(input: &mut &str) -> Result<Index> {
    digit1.parse_to().parse_next(input)
}

/// A combinator which accepts the character following a backslash, limited to
/// the escapes JSON itself defines.
fn parse_escapable(input: &mut &str) -> Result<()> {
    dispatch! {any;
        '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' => empty,
        'u' => take_while(4, AsChar::is_hex_digit).void(),
        _ => fail,
    }
    .parse_next(input)
}

/// Reads the four hexadecimal digits of a `\u` escape as a UTF-16 code unit.
fn parse_code_unit(chars: &mut Chars<'_>) -> Option<u16> {
    let mut unit: u32 = 0;

    for _ in 0..4 {
        unit = unit * 16 + chars.next()?.to_digit(16)?;
    }

    u16::try_from(unit).ok()
}

/// Appends the character a `\u` escape stands for, pairing a leading surrogate
/// with the trailing one that must follow it.
fn push_code_point(chars: &mut Chars<'_>, unescaped: &mut String) {
    let Some(leading) = parse_code_unit(chars) else {
        return;
    };

    if (0xD800..0xDC00).contains(&leading) {
        let mut lookahead = chars.clone();

        if lookahead.next() == Some('\\')
            && lookahead.next() == Some('u')
            && let Some(trailing) = parse_code_unit(&mut lookahead)
            && (0xDC00..0xE000).contains(&trailing)
            && let Some(paired) = char::from_u32(
                0x10000 + ((u32::from(leading) - 0xD800) << 10) + (u32::from(trailing) - 0xDC00),
            )
        {
            unescaped.push(paired);
            *chars = lookahead;

            return;
        }
    }

    if let Some(character) = char::from_u32(u32::from(leading)) {
        unescaped.push(character);
    }
}

/// Resolves the escape sequences of a key, borrowing the input untouched when
/// it carries none.
fn unescape(raw: &str) -> Cow<'_, str> {
    if !raw.contains('\\') {
        return Cow::Borrowed(raw);
    }

    let mut unescaped = String::with_capacity(raw.len());
    let mut chars = raw.chars();

    while let Some(character) = chars.next() {
        if character != '\\' {
            unescaped.push(character);

            continue;
        }

        match chars.next() {
            Some('b') => unescaped.push('\u{0008}'),
            Some('f') => unescaped.push('\u{000C}'),
            Some('n') => unescaped.push('\n'),
            Some('r') => unescaped.push('\r'),
            Some('t') => unescaped.push('\t'),
            Some('u') => push_code_point(&mut chars, &mut unescaped),
            Some(other) => unescaped.push(other),
            None => break,
        }
    }

    Cow::Owned(unescaped)
}

/// A combinator which parses a key surrounded by double quotes, resolving the
/// JSON escape sequences it may contain.
pub(crate) fn parse_key<'a>(input: &mut &'a str) -> Result<Cow<'a, str>> {
    trim(delimited(
        DOUBLE_QUOTE,
        take_escaped(take_till(1.., ['"', '\\']), '\\', parse_escapable).map(unescape),
        DOUBLE_QUOTE,
    ))
    .parse_next(input)
}

/// A combinator which parses a list of `Index`.
pub(crate) fn parse_indexes(input: &mut &str) -> Result<Vec<Index>> {
    separated(1.., parse_number, trim(COMMA)).parse_next(input)
}

/// A combinator which parses a list of keys.
fn parse_keys<'a>(input: &mut &'a str) -> Result<Vec<Cow<'a, str>>> {
    trim(separated(1.., parse_key, trim(COMMA))).parse_next(input)
}

/// A combinator which parses a list of keys surrounded by curly braces.
pub(crate) fn parse_multi_key<'a>(input: &mut &'a str) -> Result<Vec<Cow<'a, str>>> {
    delimited(CURLY_BRACKET_OPEN, parse_keys, CURLY_BRACKET_CLOSE).parse_next(input)
}

/// A combinator which parses an array of `Index`.
pub(crate) fn parse_array_index(input: &mut &str) -> Result<Vec<Index>> {
    delimited(
        trim(SQUARE_BRACKET_OPEN),
        parse_indexes,
        trim(SQUARE_BRACKET_CLOSE),
    )
    .parse_next(input)
}

/// A combinator which parses an array range.
pub(crate) fn parse_array_range(input: &mut &str) -> Result<(Option<Index>, Option<Index>)> {
    delimited(
        trim(SQUARE_BRACKET_OPEN),
        separated_pair(opt(parse_number), trim(COLON), opt(parse_number)),
        trim(SQUARE_BRACKET_CLOSE),
    )
    .parse_next(input)
}

/// A combinator which parses a list of index surrounded by curly braces.
pub(crate) fn parse_object_index(input: &mut &str) -> Result<Vec<Index>> {
    delimited(
        trim(CURLY_BRACKET_OPEN),
        parse_indexes,
        trim(CURLY_BRACKET_CLOSE),
    )
    .parse_next(input)
}

/// A combinator which parses an object range.
pub(crate) fn parse_object_range(input: &mut &str) -> Result<(Option<Index>, Option<Index>)> {
    delimited(
        trim(CURLY_BRACKET_OPEN),
        separated_pair(opt(parse_number), trim(COLON), opt(parse_number)),
        trim(CURLY_BRACKET_CLOSE),
    )
    .parse_next(input)
}

/// A combinator which parses any lens value.
pub(crate) fn parse_lens_value<'a>(input: &mut &'a str) -> Result<LensValue<'a>> {
    dispatch! {peek(any);
        'f' => FALSE.value(LensValue::Bool(false)),
        't' => TRUE.value(LensValue::Bool(true)),
        'n' => "null".value(LensValue::Null),
        '0'..='9' => digit1.try_map(|s: &str| s.parse::<usize>().map(LensValue::Number)),
        _ => parse_key.map(LensValue::String),
    }
    .parse_next(input)
}
//
/// A combinator which parses a lens key.
fn parse_lens_key<'a>(input: &mut &'a str) -> Result<Token<'a>> {
    trim(
        dispatch! {peek(any);
            '[' => {
                alt((
                    parse_array_index.map(Token::ArrayIndexSelector),
                    parse_array_range.map(|(start, end)| Token::ArrayRangeSelector(Range(start, end))),
                ))
            },
            '"' => parse_key.map(Token::KeySelector),
            '{' => {
                alt((
                    parse_multi_key.map(Token::MultiKeySelector),
                    parse_object_index.map(Token::ObjectIndexSelector),
                    parse_object_range.map(|(start, end)| Token::ObjectRangeSelector(Range(start, end))),
                ))
            },
            _ => fail
        }
    )
    .parse_next(input)
}

/// A combinator which parses multiple lens keys.
fn parse_lens_keys<'a>(input: &mut &'a str) -> Result<Vec<Token<'a>>> {
    repeat(1.., parse_lens_key).parse_next(input)
}

/// A combinator which parses a lens.
pub(crate) fn parse_lens<'a>(
    input: &mut &'a str,
) -> Result<(Vec<Token<'a>>, Option<LensValue<'a>>)> {
    trim((
        parse_lens_keys,
        opt(preceded(trim(EQUAL), parse_lens_value)),
    ))
    .parse_next(input)
}

/// A combinator which parses a list of lenses.
pub(crate) fn parse_lenses<'a>(
    input: &mut &'a str,
) -> Result<Vec<(Vec<Token<'a>>, Option<LensValue<'a>>)>> {
    delimited(
        trim(LENSES_START),
        separated(1.., parse_lens, trim(COMMA)),
        trim(CURLY_BRACKET_CLOSE),
    )
    .parse_next(input)
}

/// A combinator which parses a keys operator.
pub(crate) fn parse_keys_operator<'a>(input: &mut &'a str) -> Result<&'a str> {
    literal(KEYS).parse_next(input)
}

/// A combinator which parses a flatten operator.
pub(crate) fn parse_flatten_operator<'a>(input: &mut &'a str) -> Result<&'a str> {
    literal(FLATTEN).parse_next(input)
}

/// A combinator which parses a pipe in operator.
pub(crate) fn parse_pipe_in_operator<'a>(input: &mut &'a str) -> Result<&'a str> {
    literal(PIPE_IN).parse_next(input)
}

/// A combinator which parses a pipe out operator.
pub(crate) fn parse_pipe_out_operator<'a>(input: &mut &'a str) -> Result<&'a str> {
    literal(PIPE_OUT).parse_next(input)
}

/// A combinator which parses a truncate operator.
pub(crate) fn parse_truncate_operator<'a>(input: &mut &'a str) -> Result<&'a str> {
    trim(TRUNCATE).parse_next(input)
}

/// A combinator which parses a group separator.
pub(crate) fn parse_group_separator<'a>(input: &mut &'a str) -> Result<&'a str> {
    literal(GROUP_SEP).parse_next(input)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{
        FLATTEN,
        GROUP_SEP,
        KEYS,
        PIPE_IN,
        PIPE_OUT,
        TRUNCATE,
        parse_array_index,
        parse_array_range,
        parse_flatten_operator,
        parse_group_separator,
        parse_indexes,
        parse_key,
        parse_keys_operator,
        parse_lens,
        parse_lenses,
        parse_multi_key,
        parse_number,
        parse_object_index,
        parse_object_range,
        parse_pipe_in_operator,
        parse_pipe_out_operator,
        parse_truncate_operator,
    };
    use crate::tokens::{
        Index,
        LensValue,
        Token,
    };

    #[test]
    fn check_parse_number() {
        assert_eq!(parse_number(&mut "123"), Ok(Index(123)));
        assert!(parse_number(&mut "abc").is_err());
        assert!(parse_number(&mut "abc123").is_err());
    }

    #[test]
    fn check_parse_key() {
        assert_eq!(parse_key(&mut r#""abc""#), Ok(Cow::Borrowed("abc")));
        assert!(parse_key(&mut "abc").is_err());
    }
    #[test]
    fn check_parse_key_resolves_escapes() {
        assert_eq!(
            parse_key(&mut r#""a\"b""#),
            Ok(Cow::Owned("a\"b".to_string()))
        );
        assert_eq!(
            parse_key(&mut r#""a\\b""#),
            Ok(Cow::Owned("a\\b".to_string()))
        );
        assert_eq!(
            parse_key(&mut r#""a\/b""#),
            Ok(Cow::Owned("a/b".to_string()))
        );
        assert_eq!(
            parse_key(&mut r#""a\nb""#),
            Ok(Cow::Owned("a\nb".to_string()))
        );
        assert_eq!(
            parse_key(&mut r#""a\tb""#),
            Ok(Cow::Owned("a\tb".to_string()))
        );
        assert_eq!(
            parse_key(&mut r#""\u0041\u0042""#),
            Ok(Cow::Owned("AB".to_string()))
        );
        assert_eq!(
            parse_key(&mut r#""\uD83D\uDE00""#),
            Ok(Cow::Owned("\u{1F600}".to_string()))
        );
    }

    #[test]
    fn check_parse_key_owns_only_what_it_had_to_unescape() {
        assert!(matches!(parse_key(&mut r#""a\"b""#), Ok(Cow::Owned(_))));
    }

    #[test]
    fn check_parse_key_borrows_when_there_is_nothing_to_unescape() {
        assert!(matches!(
            parse_key(&mut r#""abc""#),
            Ok(Cow::Borrowed("abc"))
        ));
        assert!(matches!(parse_key(&mut r#""""#), Ok(Cow::Borrowed(""))));
    }

    #[test]
    fn check_parse_key_rejects_invalid_escapes() {
        assert!(parse_key(&mut r#""\q""#).is_err());
        assert!(parse_key(&mut r#""\u12""#).is_err());
        assert!(parse_key(&mut r#""\u12zz""#).is_err());
        assert!(parse_key(&mut r#""a\""#).is_err());
    }

    #[test]
    fn check_parse_indexes() {
        assert_eq!(parse_indexes(&mut "123"), Ok(vec![Index(123)]));
        assert_eq!(
            parse_indexes(&mut "123,456,789"),
            Ok(vec![Index(123), Index(456), Index(789)])
        );
        assert!(parse_indexes(&mut "abc").is_err());
    }

    #[test]
    fn check_parse_multi_key() {
        assert_eq!(
            parse_multi_key(&mut r#"{"abc"}"#),
            Ok(vec![Cow::Borrowed("abc")])
        );
        assert_eq!(
            parse_multi_key(&mut r#"{"abc","def"}"#),
            Ok(vec![Cow::Borrowed("abc"), Cow::Borrowed("def")])
        );
        assert!(parse_multi_key(&mut "{}").is_err());
        assert!(parse_multi_key(&mut "{123}").is_err());
    }

    #[test]
    fn check_parse_array_index() {
        assert_eq!(parse_array_index(&mut "[1]"), Ok(vec![Index(1)]));
        assert_eq!(
            parse_array_index(&mut "[1,2,3]"),
            Ok(vec![Index(1), Index(2), Index(3)])
        );
        assert!(parse_array_index(&mut "[]").is_err());
        assert!(parse_array_index(&mut r#"["1"]"#).is_err());
    }

    #[test]
    fn check_parse_array_range() {
        assert_eq!(parse_array_range(&mut "[:]"), Ok((None, None)));
        assert_eq!(parse_array_range(&mut "[1:]"), Ok((Some(Index(1)), None)));
        assert_eq!(parse_array_range(&mut "[:1]"), Ok((None, Some(Index(1)))));
        assert_eq!(
            parse_array_range(&mut "[1:3]"),
            Ok((Some(Index(1)), Some(Index(3))))
        );
        assert!(parse_array_range(&mut "[]").is_err());
    }

    #[test]
    fn check_bracketed_selectors_accept_the_same_whitespace() {
        assert_eq!(
            parse_array_index(&mut "[ 0 , 1 ]"),
            Ok(vec![Index(0), Index(1)])
        );
        assert_eq!(
            parse_array_range(&mut "[ 0 : 1 ]"),
            Ok((Some(Index(0)), Some(Index(1))))
        );
        assert_eq!(
            parse_object_index(&mut "{ 0 , 1 }"),
            Ok(vec![Index(0), Index(1)])
        );
        assert_eq!(
            parse_object_range(&mut "{ 0 : 1 }"),
            Ok((Some(Index(0)), Some(Index(1))))
        );
        assert_eq!(parse_array_range(&mut "[ : ]"), Ok((None, None)));
        assert_eq!(parse_object_range(&mut "{ : }"), Ok((None, None)));
    }

    #[test]
    fn check_parse_object_index() {
        assert_eq!(parse_object_index(&mut "{1}"), Ok(vec![Index(1)]));
        assert_eq!(
            parse_object_index(&mut "{1,2}"),
            Ok(vec![Index(1), Index(2)])
        );
        assert!(parse_object_index(&mut "{}").is_err());
    }

    #[test]
    fn check_parse_object_range() {
        assert_eq!(parse_object_range(&mut "{:}"), Ok((None, None)));
        assert_eq!(parse_object_range(&mut "{1:}"), Ok((Some(Index(1)), None)));
        assert_eq!(parse_object_range(&mut "{:1}"), Ok((None, Some(Index(1)))));
        assert_eq!(
            parse_object_range(&mut "{1:3}"),
            Ok((Some(Index(1)), Some(Index(3))))
        );
        assert!(parse_object_range(&mut "{}").is_err());
    }

    #[test]
    fn check_parse_keys_operator() {
        assert_eq!(parse_keys_operator(&mut "@"), Ok(KEYS));
        assert!(parse_keys_operator(&mut "").is_err());
    }

    #[test]
    fn check_parse_flatten_operator() {
        assert_eq!(parse_flatten_operator(&mut ".."), Ok(FLATTEN));
        assert!(parse_flatten_operator(&mut "").is_err());
    }

    #[test]
    fn check_parse_pipe_in_operator() {
        assert_eq!(parse_pipe_in_operator(&mut "|>"), Ok(PIPE_IN));
        assert!(parse_pipe_in_operator(&mut "").is_err());
    }

    #[test]
    fn check_parse_pipe_out_operator() {
        assert_eq!(parse_pipe_out_operator(&mut "<|"), Ok(PIPE_OUT));
        assert!(parse_pipe_out_operator(&mut "").is_err());
    }

    #[test]
    fn check_parse_truncate_operator() {
        assert_eq!(parse_truncate_operator(&mut "!"), Ok(TRUNCATE));
        assert!(parse_truncate_operator(&mut "").is_err());
    }

    #[test]
    fn check_parse_group_separator() {
        assert_eq!(parse_group_separator(&mut ","), Ok(GROUP_SEP));
        assert!(parse_group_separator(&mut "").is_err());
    }

    #[test]
    fn check_parse_lens() {
        assert_eq!(
            parse_lens(&mut r#""abc""#),
            Ok((vec![Token::KeySelector("abc".into())], None))
        );
        assert_eq!(
            parse_lens(&mut r#""abc"=null"#),
            Ok((
                vec![Token::KeySelector("abc".into())],
                Some(LensValue::Null)
            ))
        );
        assert_eq!(
            parse_lens(&mut r#""abc"="def""#),
            Ok((
                vec![Token::KeySelector("abc".into())],
                Some(LensValue::String("def".into()))
            ))
        );
        assert_eq!(
            parse_lens(&mut r#""abc"=123"#),
            Ok((
                vec![Token::KeySelector("abc".into())],
                Some(LensValue::Number(123))
            ))
        );
        assert_eq!(
            parse_lens(&mut r#""abc""bcd"[0]=123"#),
            Ok((
                vec![
                    Token::KeySelector("abc".into()),
                    Token::KeySelector("bcd".into()),
                    Token::ArrayIndexSelector(vec![Index(0)])
                ],
                Some(LensValue::Number(123))
            ))
        );
        assert!(parse_lens(&mut "").is_err());
    }

    #[test]
    fn check_parse_lenses() {
        assert_eq!(
            parse_lenses(&mut r#"|={"abc","bcd"=123,"efg"=null,"hij"="test"}"#),
            Ok(vec![
                (vec![Token::KeySelector("abc".into())], None),
                (
                    vec![Token::KeySelector("bcd".into())],
                    Some(LensValue::Number(123))
                ),
                (
                    vec![Token::KeySelector("efg".into())],
                    Some(LensValue::Null)
                ),
                (
                    vec![Token::KeySelector("hij".into())],
                    Some(LensValue::String("test".into()))
                ),
            ])
        );
    }
}
