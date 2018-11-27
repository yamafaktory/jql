extern crate regex;
extern crate serde_json;

use group_walker::group_walker;
use parser::selectors_parser;
use serde_json::json;
use serde_json::Value;

/// Given some selectors walk over the JSON file.
pub fn walker(json: &Value, selectors: Option<&str>) -> Result<Value, String> {
    // A Selector has been found.
    if let Some(selectors) = selectors {
        return match selectors_parser(selectors) {
            Ok(groups) => {
                // Capture groups separated by commas, apply the selector for the
                // current group and return a Result of values or an Err early on.
                let inner_groups: Result<Vec<Value>, String> = groups
                    .iter()
                    .map(|group| group_walker(group.clone(), json))
                    .collect();
                match inner_groups {
                    Ok(groups) => match groups.len() {
                        0 => Err(String::from("Empty selection")),
                        // One group.
                        1 => Ok(json!(groups[0])),
                        // Multiple groups.
                        _ => Ok(json!(groups)),
                    },
                    Err(error) => Err(error),
                }
            }
            Err(error) => Err(error),
        };
    }
    // Nothing found.
    Err(String::from("No selector found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // The following constants are all valid JSON.
    // https://tools.ietf.org/html/rfc8259#section-13

    const SINGLE_VALUE_DATA: &str = r#"1337"#;

    const SINGLE_NULL_VALUE_DATA: &str = r#"null"#;

    const ARRAY_DATA: &str = r#"[1, 2, 3, null]"#;

    const DATA: &str = r#"{
        "array": [1, 2, 3, null],
        "nested": {
            "a": "one",
            "b": "two",
            "c": "three"
        },
        "null": null,
        "number": 1337,
        "text": "some text",
        ".property..": "This is valid JSON!",
        "\"": "This is valid JSON as well",
        " ": "Yup, this too 🐼!",
        "": "Yup, again 🐨!",
        "mix": [{ "first": 1 }],
        "range": [1, 2, 3, 4, 5, 6, 7],
        "filter": [
            { "color": "red" },
            { "color": "green" },
            { "color": "blue" }
        ],
        "nested-filter": [
            {
                "laptop": {
                    "brand": "Apple",
                    "options": ["a", "b", "c"]
                }
            },
            {
                "laptop": {
                    "brand": "Asus",
                    "options": ["d", "e", "f"]
                }
            }
        ],
        "filter-to-flatten": [[[[["c", "a", "c"]]]], "g", [[["a", ["t"]]]]],
        "nested-filter-to-flatten": [
            {
                "fruit": {
                    "type": "Banana",
                    "dna": ["c", "g", "a", "t"]
                }
            },
            {
                "fruit": {
                    "type": "Pear",
                    "dna": [[[[["c", "a", "c"]]]], "g", "t", [[["a", ["t"]]]]]
                }
            }
        ]
    }"#;

    #[test]
    fn get_text() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("text");
        assert_eq!(Ok(json["text"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_number() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("number");
        assert_eq!(Ok(json["number"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_null() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("null");
        assert_eq!(Ok(Value::Null), walker(&json, selector));
    }

    #[test]
    fn get_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array");
        assert_eq!(Ok(json["array"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array.0");
        assert_eq!(Ok(json["array"][0].clone()), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array.4");
        assert_eq!(
            Err(String::from(
                "Index ( 4 ) is out of bound, node ( array ) has a length of 4"
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_out_of_bound_item_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector = Some("4");
        assert_eq!(
            Err(String::from(
                "Index ( 4 ) is out of bound, root element has a length of 4"
            )),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_negative_index_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array.-1");
        assert_eq!(
            Err(String::from("Invalid negative array index")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_negative_index_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector = Some("-1");
        assert_eq!(
            Err(String::from("Invalid negative array index")),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_null_in_array() {
        let json_array: Value = serde_json::from_str(DATA).unwrap();
        let array_selector = Some("array.3");
        assert_eq!(Ok(Value::Null), walker(&json_array, array_selector));
    }

    #[test]
    fn get_null_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector = Some("3");
        assert_eq!(Ok(Value::Null), walker(&json_array, array_selector));
    }

    #[test]
    fn get_index_in_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("text.1");
        let root_selector = Some("1");
        let root_selector_nested = Some("0.1");
        assert_eq!(
            Err(String::from("Node ( text ) is not an array")),
            walker(&json, selector)
        );
        assert_eq!(
            Err(String::from("Root element is not an array")),
            walker(&json, root_selector)
        );
        assert_eq!(
            Err(String::from("Root element is not an array")),
            walker(&json, root_selector_nested)
        );
    }

    #[test]
    fn get_non_existing_node_on_root() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("foo");
        assert_eq!(
            Err(String::from("Node ( foo ) not found on the parent element")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested.d");
        assert_eq!(
            Err(String::from(
                "Node ( d ) not found on parent node ( nested )"
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested.a");
        assert_eq!(Ok(json["nested"]["a"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_single_value() {
        let json_single_value: Value =
            serde_json::from_str(SINGLE_VALUE_DATA).unwrap();
        let selector = Some(".");
        assert_eq!(
            Ok(json_single_value.clone()),
            walker(&json_single_value, selector)
        );
    }

    #[test]
    fn get_single_null_value() {
        let json_single_value: Value =
            serde_json::from_str(SINGLE_NULL_VALUE_DATA).unwrap();
        let selector = Some(".");
        assert_eq!(Ok(Value::Null), walker(&json_single_value, selector));
    }

    #[test]
    fn get_empty() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("");
        assert_eq!(
            Err(String::from("Empty selection")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_raw_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some(".");
        assert_eq!(Ok(json.clone()), walker(&json, selector));
    }

    #[test]
    fn get_weird_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let dot_selector = Some(r#"".property..""#);
        let quote_selector = Some(r#"""""#);
        let space_selector = Some(r#"" ""#);
        let empty_selector = Some(r#""""#);
        assert_eq!(
            Ok(json[".property.."].clone()),
            walker(&json, dot_selector)
        );
        assert_eq!(Ok(json[r#"""#].clone()), walker(&json, quote_selector));
        assert_eq!(Ok(json[r#" "#].clone()), walker(&json, space_selector));
        assert_eq!(Ok(json[r#""#].clone()), walker(&json, empty_selector));
    }

    #[test]
    fn get_mix_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let mix_selector = Some("mix.0.first");
        assert_eq!(
            Ok(json["mix"][0]["first"].clone()),
            walker(&json, mix_selector)
        );
    }

    #[test]
    fn get_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("range.2:5");
        assert_eq!(Ok(json!([3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_one_item_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("range.2:2");
        assert_eq!(Ok(json!([3])), walker(&json, selector));
    }

    #[test]
    fn get_reversed_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("range.5:2");
        assert_eq!(Ok(json!([6, 5, 4, 3])), walker(&json, selector));
    }

    #[test]
    fn get_original_from_reversed_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("range.5:2.3:0");
        assert_eq!(Ok(json!([3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("range.6:7");
        assert_eq!(
            Err(String::from("Range ( 6 : 7 ) is out of bound, node ( range ) has a length of 7")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_range_on_non_array_root() {
        let json: Value = serde_json::from_str(SINGLE_VALUE_DATA).unwrap();
        let selector = Some("2:0");
        assert_eq!(
            Err(String::from("Root element is not an array")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_range_on_non_array_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested.0.1");
        assert_eq!(
            Err(String::from("Node ( nested ) is not an array")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_multi_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array,number");
        assert_eq!(
            Ok(json!([json["array"], json["number"]])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_multi_selection_with_space() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array,,, ");
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_multi_selection_with_empty() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("array,,,");
        assert_eq!(Ok(json["array"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_multi_selection_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("range.5:3,array.2:1");
        assert_eq!(Ok(json!([[6, 5, 4], [3, 2]])), walker(&json, selector));
    }

    #[test]
    fn get_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("filter|color");
        assert_eq!(
            Ok(json!(["red", "green", "blue"])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_only_one_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("|color|color");
        assert_eq!(
            Err(String::from(
                "Node ( |color ) not found on the parent element"
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_wrong_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("filter|colors");
        assert_eq!(
            Err(String::from(
                "Node ( colors ) not found on the parent element"
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_filter_with_no_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("|color");
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_filter_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("filter.1:2|color");
        assert_eq!(Ok(json!(["green", "blue"])), walker(&json, selector));
    }

    #[test]
    fn get_wrong_filter_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("filter.1:2|colors");
        assert_eq!(
            Err(String::from(
                "Node ( colors ) not found on the parent element"
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_filter_with_multi_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("filter.1:2|color,filter.2:1|color");
        assert_eq!(
            Ok(json!([["green", "blue"], ["blue", "green"]])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_nested_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested-filter|laptop.brand");
        assert_eq!(Ok(json!(["Apple", "Asus"])), walker(&json, selector));
    }

    #[test]
    fn get_nested_filter_with_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested-filter|laptop.options.0");
        assert_eq!(Ok(json!(["a", "d"])), walker(&json, selector));
    }

    #[test]
    fn get_nested_filter_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested-filter|laptop.options.1:2");
        assert_eq!(
            Ok(json!([["b", "c"], ["e", "f"]])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_filter_on_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("nested|some");
        assert_eq!(
            Err(String::from("A filter can only be applied to an array")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_flattened_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = Some("..filter-to-flatten");
        assert_eq!(Ok(json!(["Apple", "Asus"])), walker(&json, selector));
    }
}
