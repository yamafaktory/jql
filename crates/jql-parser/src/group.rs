use crate::tokens::Token;

/// Splits a list of `Tokens` by `GroupSeparator`.
/// Returns groups of `Tokens`.
pub fn split<'a>(tokens: &'a [Token<'a>]) -> Vec<Vec<&'a Token<'a>>> {
    tokens
        .iter()
        .fold(vec![], |mut acc: Vec<Vec<&Token>>, token| {
            if token == &Token::GroupSeparator {
                acc.push(vec![]);

                return acc;
            }

            if acc.is_empty() {
                acc.push(vec![]);
            }

            if let Some(last) = acc.last_mut() {
                last.push(token);
            }

            acc
        })
        .into_iter()
        .filter(|group| !group.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::split;
    use crate::tokens::Token;

    #[test]
    fn check_split() {
        assert!(split(&[Token::GroupSeparator,]).is_empty());
        assert_eq!(
            split(&[Token::KeySelector("abc")]),
            vec![vec![&Token::KeySelector("abc")]]
        );
        assert_eq!(
            split(&[
                Token::GroupSeparator,
                Token::GroupSeparator,
                Token::KeySelector("abc")
            ]),
            vec![vec![&Token::KeySelector("abc")]]
        );
        assert_eq!(
            split(&[
                Token::KeySelector("abc"),
                Token::GroupSeparator,
                Token::KeySelector("abc")
            ]),
            vec![
                vec![&Token::KeySelector("abc")],
                vec![&Token::KeySelector("abc")],
            ]
        );
    }
}
