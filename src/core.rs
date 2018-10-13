use regex::Regex;
use serde_json::Value;
use types::Selection;

/// Get the trimmed text of the match with a default of an empty
/// string if the group didn't participate in the match.
fn get_selector(capture: regex::Captures<'_>) -> String {
    let cap = capture.get(0).map_or("", |m| m.as_str()).trim();
    if cap.starts_with('\"') {
        let cap_string = String::from(cap);
        // Drop the enclosing double quotes in this case.
        let inner_cap = &cap_string[1..cap_string.len() - 1];
        String::from(inner_cap)
    } else {
        String::from(cap)
    }
}

pub fn walker(json: &Value, selector: Option<&str>) -> Option<Selection> {
    let mut inner_json = json;
    if let Some(selector) = selector {
        // Capture groups of double quoted selectors and simple ones surrounded
        // by dots.
        let re = Regex::new(r#"("[^"]+")|([^.]+)"#).unwrap();
        let selector: Vec<String> =
            re.captures_iter(selector).map(get_selector).collect();

        // Returns a Result of values or an Err early on, stopping the iteration
        // as soon as the latter is encountered.
        let items: Selection = selector
            .iter()
            .enumerate()
            .map(|(i, s)| -> Result<Value, String> {
                // Array case.
                if let Ok(index) = s.parse::<isize>() {
                    // A Negative index has been provided.
                    if (index).is_negative() {
                        return Err(String::from(
                            "Invalid negative array index",
                        ));
                    }

                    // A JSON null value has been found (array).
                    if inner_json[index as usize] == Value::Null {
                        let error_message = match inner_json.as_array() {
                            // Trying to access an out of bound index on a node
                            // or on the root element.
                            Some(array) => {
                                if selector.len() == 1 {
                                    [
                                        "Index (",
                                        s,
                                        ") is out of bound, root element has \
                                         a length of",
                                        &(array.len()).to_string(),
                                    ]
                                        .join(" ")
                                } else {
                                    [
                                        "Index (",
                                        s,
                                        ") is out of bound, node (",
                                        selector[i - 1].as_str(),
                                        ") has a length of",
                                        &(array.len()).to_string(),
                                    ]
                                        .join(" ")
                                }
                            }
                            // Trying to access an index on a node which is not
                            // an array.
                            None => {
                                if selector.len() == 1 {
                                    ["Root element is not an array"].join(" ")
                                } else {
                                    [
                                        "Node (",
                                        selector[i - 1].as_str(),
                                        ") is not an array",
                                    ]
                                        .join(" ")
                                }
                            }
                        };

                        return Err(error_message);
                    }

                    // Match found.
                    inner_json = &inner_json[index as usize];
                    return Ok(inner_json.clone());
                }

                // A JSON null value has been found (non array).
                if inner_json[s] == Value::Null {
                    if i == 0 {
                        Err(["Node (", s, ") is not the root element"]
                            .join(" "))
                    } else {
                        Err([
                            "Node (",
                            s,
                            ") not found on parent (",
                            selector[i - 1].as_str(),
                            ")",
                        ]
                            .join(" "))
                    }
                } else {
                    inner_json = &inner_json[s];
                    Ok(inner_json.clone())
                }
            })
            .collect();

        // Final check for empty selection, in this case we assume that the user
        // expects to get back the complete raw JSON back.
        Some(match items {
            Ok(items) => {
                if items.is_empty() {
                    Ok(vec![json.clone()])
                } else {
                    Ok(items)
                }
            }
            Err(items) => Err(items),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The following constants are all valid JSON.
    // https://tools.ietf.org/html/rfc8259#section-13

    const SINGLE_VALUE_DATA: &str = r#"1337"#;

    const ARRAY_DATA: &str = r#"[1, 2, 3]"#;

    const DATA: &str = r#"{
        "array": [1, 2, 3],
        "nested": {
            "a": "one",
            "b": "two",
            "c": "three"
        },
        "number": 1337,
        "text": "some text",
        ".property..": "This is valid JSON!",
        "\"": "This is valid JSON as well"
    }"#;

    #[test]
    fn get_text() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("text");
        assert_eq!(
            Some(Ok(vec![json["text"].clone()])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_number() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("number");
        assert_eq!(
            Some(Ok(vec![json["number"].clone()])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array");
        assert_eq!(
            Some(Ok(vec![json["array"].clone()])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.0");
        assert_eq!(
            Some(Ok(vec![json["array"].clone(), json["array"][0].clone()])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_out_of_bound_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.3");
        assert_eq!(
            Some(Err(String::from(
                "Index ( 3 ) is out of bound, node ( array ) has a length of 3"
            ))),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_out_of_bound_item_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector: Option<&str> = Some("3");
        assert_eq!(
            Some(Err(String::from(
                "Index ( 3 ) is out of bound, root element has a length of 3"
            ))),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_negative_index_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.-1");
        assert_eq!(
            Some(Err(String::from("Invalid negative array index"))),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_negative_index_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector: Option<&str> = Some("-1");
        assert_eq!(
            Some(Err(String::from("Invalid negative array index"))),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_index_in_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("text.1");
        let root_selector: Option<&str> = Some("1");
        assert_eq!(
            Some(Err(String::from("Node ( text ) is not an array"))),
            walker(&json, selector)
        );
        assert_eq!(
            Some(Err(String::from("Root element is not an array"))),
            walker(&json, root_selector)
        );
    }

    #[test]
    fn get_non_existing_root_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("foo");
        assert_eq!(
            Some(Err(String::from("Node ( foo ) is not the root element"))),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("nested.d");
        assert_eq!(
            Some(Err(String::from(
                "Node ( d ) not found on parent ( nested )"
            ))),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("nested.a");
        assert_eq!(
            Some(Ok(vec![
                json["nested"].clone(),
                json["nested"]["a"].clone()
            ])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_single_value() {
        let json_single_value: Value =
            serde_json::from_str(SINGLE_VALUE_DATA).unwrap();
        let selector: Option<&str> = Some(".");
        assert_eq!(
            Some(Ok(vec![json_single_value.clone()])),
            walker(&json_single_value, selector)
        );
    }

    #[test]
    fn get_raw_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("");
        assert_eq!(Some(Ok(vec![json.clone()])), walker(&json, selector));
    }

    #[test]
    fn get_weird_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let dot_selector: Option<&str> = Some(r#"".property..""#);
        let quote_selector: Option<&str> = Some(r#"""""#);
        assert_eq!(
            Some(Ok(vec![json[".property.."].clone()])),
            walker(&json, dot_selector)
        );
        assert_eq!(
            Some(Ok(vec![json[r#"""#].clone()])),
            walker(&json, quote_selector)
        );
    }
}
