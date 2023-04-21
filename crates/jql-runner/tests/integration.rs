use jql_parser::tokens::Token;
use jql_runner::runner::{
    raw,
    token,
};
use serde_json::json;

#[test]
fn check_raw_integration() {
    assert_eq!(
        raw(r#""a","b""#, &json!({ "a": 1, "b": 2 })),
        Ok(json!([1, 2]))
    );
}

#[test]
fn check_token_integration() {
    assert_eq!(
        token(
            &[
                Token::KeySelector("a"),
                Token::GroupSeparator,
                Token::KeySelector("b")
            ],
            &json!({ "a": 1, "b": 2 })
        ),
        Ok(json!([1, 2]))
    );
}
