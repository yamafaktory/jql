#![allow(missing_docs)]

use jql_parser::{
    parser::parse,
    tokens::{
        Index,
        Token,
    },
};

#[test]
fn check_parse_integration() {
    assert_eq!(
        parse(r#""this"[9,0]|>"some"<|"ok"..!"#),
        Ok(vec![
            Token::KeySelector("this".into()),
            Token::ArrayIndexSelector(vec![Index::new(9), Index::new(0)]),
            Token::PipeInOperator,
            Token::KeySelector("some".into()),
            Token::PipeOutOperator,
            Token::KeySelector("ok".into()),
            Token::FlattenOperator,
            Token::TruncateOperator
        ]),
    );
}
