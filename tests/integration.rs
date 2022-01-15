#![deny(unsafe_code, nonstandard_style)]

use jql::walker;
use serde_json::json;

#[test]
fn integration() {
    let json_array = json!([2, 3, 5, 7, 11]);

    assert_eq!(walker(&json_array, Some("[4]")), Ok(json!(11)));
}
