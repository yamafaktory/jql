extern crate jql;
extern crate serde_json;

use serde_json::Value;

const DATA: &str = r#"{
    "text": "some text",
    "number": 11,
    "array": [1,2,3]
}"#;

#[test]
fn get_text() {
    let json: Value = serde_json::from_str(DATA).unwrap();
    let selector: Option<&str> = Some("text");
    assert_eq!(
        Some(Ok(vec![json["text"].clone()])),
        jql::core::walker(&json, selector)
    );
}

#[test]
fn get_number() {
    let json: Value = serde_json::from_str(DATA).unwrap();
    let selector: Option<&str> = Some("number");
    assert_eq!(
        Some(Ok(vec![json["number"].clone()])),
        jql::core::walker(&json, selector)
    );
}

#[test]
fn get_array() {
    let json: Value = serde_json::from_str(DATA).unwrap();
    let selector: Option<&str> = Some("array");
    assert_eq!(
        Some(Ok(vec![json["array"].clone()])),
        jql::core::walker(&json, selector)
    );
}

#[test]
fn get_array_item() {
    let json: Value = serde_json::from_str(DATA).unwrap();
    let selector: Option<&str> = Some("array.0");
    assert_eq!(
        Some(Ok(vec![json["array"].clone(), json["array"][0].clone()])),
        jql::core::walker(&json, selector)
    );
}
