#![deny(unsafe_code, nonstandard_style)]
#![forbid(rust_2021_compatibility)]

use jql::{
    groups_walker, selectors_parser, walker, Group,
    Selector::{Default, Index, Range},
};
use serde_json::json;

#[test]
fn integration_selectors_parser() {
    let selector = r#""range".[5:3],"array".[2:1]"#;

    assert_eq!(
        selectors_parser(selector),
        Ok(vec![
            Group {
                filters: vec![],
                filter_lenses: vec![],
                root: None,
                selectors: vec![Default(String::from("range")), Range((Some(5), Some(3)))],
                spread: None,
                truncate: None,
            },
            Group {
                filters: vec![],
                filter_lenses: vec![],
                root: None,
                selectors: vec![Default(String::from("array")), Range((Some(2), Some(1)))],
                spread: None,
                truncate: None,
            }
        ])
    );
}

#[test]
fn integration_groups_walker() {
    let json_array = json!([2, 3, 5, 7, 11]);

    assert_eq!(
        groups_walker(
            &json_array,
            &[Group {
                filters: vec![],
                filter_lenses: vec![],
                root: None,
                selectors: vec![Index(vec![4])],
                spread: None,
                truncate: None,
            }]
        ),
        Ok(json!(11))
    );
}

#[test]
fn integration_walker() {
    let json_array = json!([2, 3, 5, 7, 11]);

    assert_eq!(walker(&json_array, "[4]"), Ok(json!(11)));
}
