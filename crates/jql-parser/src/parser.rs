use nom::{
    branch::alt,
    combinator::{
        iterator,
        map,
        value,
    },
    IResult,
};

use crate::{
    combinators::{
        parse_array_index,
        parse_array_range,
        parse_flatten_operator,
        parse_group_separator,
        parse_key,
        parse_lenses,
        parse_multi_key,
        parse_object_index,
        parse_object_range,
        parse_pipe_in_operator,
        parse_pipe_out_operator,
        parse_truncate_operator,
    },
    errors::JqlParserError,
    tokens::{
        Lens,
        Range,
        Token,
        View,
    },
};

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
        map(parse_lenses(), |lenses| {
            Token::LensSelector(
                lenses
                    .into_iter()
                    .map(|(key, value)| Lens(key, value))
                    .collect(),
            )
        }),
        value(Token::FlattenOperator, parse_flatten_operator()),
        value(Token::GroupSeparator, parse_group_separator()),
        value(Token::PipeInOperator, parse_pipe_in_operator()),
        value(Token::PipeOutOperator, parse_pipe_out_operator()),
        value(Token::TruncateOperator, parse_truncate_operator()),
    ))(input)
}

/// Parses the provided input and returns a vector of `Tokens`.
///
/// # Errors
///
/// Returns a `JqlParserError` on failure.
pub fn parse(input: &str) -> Result<Vec<Token>, JqlParserError> {
    let mut parser_iterator = iterator(input, parse_fragment);
    let tokens = parser_iterator.collect::<Vec<Token>>();
    let result: IResult<_, _> = parser_iterator.finish();

    match result {
        Ok((unparsed, _)) => {
            if !unparsed.is_empty() {
                return Err(JqlParserError::ParsingError {
                    tokens: tokens.stringify(),
                    unparsed: unparsed.to_string(),
                });
            }

            let truncate_count = tokens
                .iter()
                .filter(|&token| *token == Token::TruncateOperator)
                .count();

            if truncate_count > 1
                || (truncate_count == 1 && tokens.last() != Some(&Token::TruncateOperator))
            {
                return Err(JqlParserError::TruncateError(tokens.stringify()));
            }

            Ok(tokens)
        }
        Err(_) => Err(JqlParserError::UnknownError),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        parse,
        parse_fragment,
    };
    use crate::{
        errors::JqlParserError,
        tokens::{
            Index,
            Lens,
            LensValue,
            Range,
            Token,
            View,
        },
    };

    #[test]
    fn check_array_index_selector() {
        assert_eq!(
            parse_fragment("[0,1,2]"),
            Ok((
                "",
                Token::ArrayIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
        assert_eq!(
            parse_fragment(" [ 0 , 1 , 2 ] "),
            Ok((
                "",
                Token::ArrayIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
    }

    #[test]
    fn check_array_range_selector() {
        assert_eq!(
            parse_fragment("[0:2]"),
            Ok((
                "",
                Token::ArrayRangeSelector(Range(Some(Index(0)), Some(Index(2))))
            ))
        );
        assert_eq!(
            parse_fragment("[:2]"),
            Ok(("", Token::ArrayRangeSelector(Range(None, Some(Index(2))))))
        );
        assert_eq!(
            parse_fragment("[0:]"),
            Ok(("", Token::ArrayRangeSelector(Range(Some(Index(0)), None))))
        );
        assert_eq!(
            parse_fragment("[:]"),
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
            parse_fragment("{0,1,2}"),
            Ok((
                "",
                Token::ObjectIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
        assert_eq!(
            parse_fragment(" { 0 , 1 , 2 } "),
            Ok((
                "",
                Token::ObjectIndexSelector(vec![Index(0), Index(1), Index(2)])
            ))
        );
    }

    #[test]
    fn check_object_range_selector() {
        assert_eq!(
            parse_fragment("{0:2}"),
            Ok((
                "",
                Token::ObjectRangeSelector(Range(Some(Index(0)), Some(Index(2))))
            ))
        );
        assert_eq!(
            parse_fragment("{:2}"),
            Ok(("", Token::ObjectRangeSelector(Range(None, Some(Index(2))))))
        );
        assert_eq!(
            parse_fragment("{0:}"),
            Ok(("", Token::ObjectRangeSelector(Range(Some(Index(0)), None))))
        );
        assert_eq!(
            parse_fragment("{:}"),
            Ok(("", Token::ObjectRangeSelector(Range(None, None))))
        );
    }

    #[test]
    fn check_lens_selector() {
        assert_eq!(
            parse_fragment(r#"|={"abc","bcd"=123,"efg"=null,"hij"="test"}"#),
            Ok((
                "",
                Token::LensSelector(vec![
                    Lens("abc", None),
                    Lens("bcd", Some(LensValue::Number(123))),
                    Lens("efg", Some(LensValue::Null)),
                    Lens("hij", Some(LensValue::String("test"))),
                ])
            ))
        );
    }

    #[test]
    fn check_flatten_operator() {
        assert_eq!(parse_fragment(".."), Ok(("", Token::FlattenOperator)));
        assert_eq!(parse_fragment(" .. "), Ok(("", Token::FlattenOperator)));
    }

    #[test]
    fn check_pipe_in_operator() {
        assert_eq!(parse_fragment("|>"), Ok(("", Token::PipeInOperator)));
        assert_eq!(parse_fragment(" |> "), Ok(("", Token::PipeInOperator)));
    }

    #[test]
    fn check_pipe_out_operator() {
        assert_eq!(parse_fragment("<|"), Ok(("", Token::PipeOutOperator)));
        assert_eq!(parse_fragment(" <| "), Ok(("", Token::PipeOutOperator)));
    }

    #[test]
    fn check_truncate_operator() {
        assert_eq!(parse_fragment("!"), Ok(("", Token::TruncateOperator)));
        assert_eq!(parse_fragment(" ! "), Ok(("", Token::TruncateOperator)));
    }

    #[test]
    fn check_group_separator() {
        assert_eq!(parse_fragment(","), Ok(("", Token::GroupSeparator)));
        assert_eq!(parse_fragment(" , "), Ok(("", Token::GroupSeparator)));
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
            parse("[9,0]nope"),
            Err(JqlParserError::ParsingError {
                tokens: [Token::ArrayIndexSelector(vec![Index(9), Index(0)])].stringify(),
                unparsed: "nope".to_string(),
            })
        );
        assert_eq!(
            parse(r#""this"[9,0]|>"some"<|"ok"..!"#),
            Ok(vec![
                Token::KeySelector("this"),
                Token::ArrayIndexSelector(vec![Index(9), Index(0)]),
                Token::PipeInOperator,
                Token::KeySelector("some"),
                Token::PipeOutOperator,
                Token::KeySelector("ok"),
                Token::FlattenOperator,
                Token::TruncateOperator
            ]),
        );
        assert_eq!(
            parse(r#""a"!"b""#),
            Err(JqlParserError::TruncateError(
                [
                    Token::KeySelector("a"),
                    Token::TruncateOperator,
                    Token::KeySelector("b")
                ]
                .stringify()
            ))
        )
    }
}
