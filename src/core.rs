extern crate serde_json;

use serde_json::Value;
use types::Selection;

pub fn walker(json: &Value, selector: Option<&str>) -> Option<Selection> {
    let mut inner_json = json;
    if let Some(selector) = selector {
        let selector: Vec<&str> = selector.split('.').collect();
        // Returns Result of values or Err early on, stopping the iteration.
        let items: Selection = selector
            .iter()
            .enumerate()
            .map(|(i, s)| -> Result<Value, String> {
                if let Ok(index) = s.parse::<isize>() {
                    if (index).is_negative() {
                        Err(String::from("Invalid negative array index"))
                    } else if inner_json[index as usize] == Value::Null {
                        let error_message = match inner_json.as_array() {
                            Some(array) => [
                                "Index (",
                                s,
                                ") is out of bound, node (",
                                selector[i - 1],
                                ") has a length of",
                                &(array.len()).to_string(),
                            ]
                                .join(" "),
                            // Trying to access an index on a node which is not
                            // an array.
                            None => {
                                ["Node (", selector[i - 1], ") is not an array"]
                                    .join(" ")
                            }
                        };
                        println!("# {:?} #", inner_json.as_array());
                        Err(error_message)
                    } else {
                        inner_json = &inner_json[index as usize];
                        Ok(inner_json.clone())
                    }
                } else if s.is_empty() {
                    Err(String::from("Unterminated selector found"))
                } else if inner_json[s] == Value::Null {
                    if i == 0 {
                        Err(["Node (", s, ") is not the root element"]
                            .join(" "))
                    } else {
                        Err([
                            "Node (",
                            s,
                            ") not found on parent (",
                            selector[i - 1],
                            ")",
                        ]
                            .join(" "))
                    }
                } else {
                    inner_json = &inner_json[s];
                    Ok(inner_json.clone())
                }
            }).collect();
        Some(items)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"{
        "array": [1,2,3],
        "nested": {
            "a": "one",
            "b": "two",
            "c": "three"
        },
        "number": 1337,
        "text": "some text"
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
    fn get_negative_index_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.-1");
        assert_eq!(
            Some(Err(String::from("Invalid negative array index"))),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_index_in_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("text.1");
        assert_eq!(
            Some(Err(String::from("Node ( text ) is not an array"))),
            walker(&json, selector)
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
    fn get_unterminated_selector() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("nested.");
        assert_eq!(
            Some(Err(String::from("Unterminated selector found"))),
            walker(&json, selector)
        );
    }
}
