use crate::{
    group_walker::group_walker,
    parser::selectors_parser,
    types::{Group, Selection, Selections},
};

use rayon::prelude::*;
use serde_json::{json, Value};

/// Walks over the Serde JSON value based on the provided selectors.
pub fn walker(json: &Value, selectors: &str) -> Selection {
    match selectors_parser(selectors) {
        Ok(groups) => groups_walker(json, &groups),
        Err(error) => Err(error),
    }
}

/// Walks over the Serde JSON value based on the provided groups.
pub fn groups_walker(json: &Value, groups: &[Group]) -> Selection {
    // Capture groups separated by commas, return a Result of values
    // or an Err early on.
    let inner_groups: Selections = groups
        .par_iter()
        .map(|group| group_walker(group, json))
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

#[cfg(test)]
mod tests {
    use super::*;

    // The following constants are all valid JSON.
    // https://tools.ietf.org/html/rfc8259#section-13

    const SINGLE_VALUE_DATA: &str = r#"1337"#;

    const SINGLE_NULL_VALUE_DATA: &str = r#"null"#;

    const ARRAY_DATA: &str = r#"[1, 2, 3, null]"#;

    const OBJECT_DATA: &str = r#"{ "a": 7, "b": 11 }"#;

    const DATA: &str = r#"{
        "array": [1, 2, 3, null],
        "empty-array": [],
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
                    "options": ["a", "b", "c"],
                    "price": 9999
                }
            },
            {
                "laptop": {
                    "brand": "Asus",
                    "options": ["d", "e", "f"],
                    "price": 999
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
        ],
        "lenses": [
            { "alpha": 1, "beta": null },
            { "beta": 2 },
            { "gamma": 3, "delta": "something" },
            { "alpha": 7 },
            { "delta": 4 }  
        ]
    }"#;

    #[test]
    fn get_text() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""text""#;
        assert_eq!(Ok(json["text"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_number() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""number""#;
        assert_eq!(Ok(json["number"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_null() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""null""#;
        assert_eq!(Ok(Value::Null), walker(&json, selector));
    }

    #[test]
    fn get_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array""#;
        assert_eq!(Ok(json["array"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_empty_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""empty-array""#;
        assert_eq!(Ok(json["empty-array"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[0]"#;
        assert_eq!(Ok(json["array"][0].clone()), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[4]"#;
        assert_eq!(
            Err(String::from(
                r#"Index [4] is out of bound, node "array" has a length of 4"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_out_of_bound_item_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector = r#"[4]"#;
        assert_eq!(
            Err(String::from(
                "Index [4] is out of bound, root element has a length of 4"
            )),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_null_in_array() {
        let json_array: Value = serde_json::from_str(DATA).unwrap();
        let array_selector = r#""array".[3]"#;
        assert_eq!(Ok(Value::Null), walker(&json_array, array_selector));
    }

    #[test]
    fn get_null_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector = r#"[3]"#;
        assert_eq!(Ok(Value::Null), walker(&json_array, array_selector));
    }

    #[test]
    fn get_index_in_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""text".[1]"#;
        let root_selector = "[1]";
        let root_selector_nested = r#"[0].[1]"#;
        assert_eq!(
            Err(String::from(r#"Node "text" is not an array"#)),
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
    fn get_root_array_without_index() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector = r#"[]"#;
        assert_eq!(Ok(json_array.clone()), walker(&json_array, array_selector));
    }

    #[test]
    fn get_array_without_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[]"#;
        assert_eq!(Ok(json!([1, 2, 3, null])), walker(&json, selector));
    }

    #[test]
    fn get_empty_array_without_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""empty-array".[]"#;
        assert_eq!(Ok(json!([])), walker(&json, selector));
    }

    #[test]
    fn get_array_of_array_without_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[].[]"#;
        assert_eq!(Ok(json!([1, 2, 3, null])), walker(&json, selector));
    }

    #[test]
    fn get_index_of_array_without_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[].[0]"#;
        assert_eq!(Ok(json!(1)), walker(&json, selector));
    }

    #[test]
    fn get_indexes_of_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[3,2,1]"#;
        assert_eq!(Ok(json!([null, 3, 2])), walker(&json, selector));
    }

    #[test]
    fn get_indexes_of_array_without_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[].[3,2,1]"#;
        assert_eq!(Ok(json!([null, 3, 2])), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_indexes_of_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array".[3,2,10]"#;
        assert_eq!(
            Err(String::from(
                r#"Index [10] is out of bound, node "array" has a length of 4"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_node_on_root() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""foo""#;
        assert_eq!(
            Err(String::from(
                r#"Node "foo" not found on the parent element"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested"."d""#;
        assert_eq!(
            Err(String::from(
                r#"Node "d" not found on parent node "nested""#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested"."a""#;
        assert_eq!(Ok(json["nested"]["a"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_single_value() {
        let json_single_value: Value = serde_json::from_str(SINGLE_VALUE_DATA).unwrap();
        let selector = ".";
        assert_eq!(
            Ok(json_single_value.clone()),
            walker(&json_single_value, selector)
        );
    }

    #[test]
    fn get_single_null_value() {
        let json_single_value: Value = serde_json::from_str(SINGLE_NULL_VALUE_DATA).unwrap();
        let selector = ".";
        assert_eq!(Ok(Value::Null), walker(&json_single_value, selector));
    }

    #[test]
    fn get_empty() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = "";
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_raw_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = ".";
        assert_eq!(Ok(json.clone()), walker(&json, selector));
    }

    #[test]
    fn get_weird_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let dot_selector = r#"".property..""#;
        let quote_selector = r##""\"""##;
        let space_selector = r#"" ""#;
        let empty_selector = r#""""#;
        assert_eq!(Ok(json[".property.."].clone()), walker(&json, dot_selector));
        assert_eq!(Ok(json[r#"""#].clone()), walker(&json, quote_selector));
        assert_eq!(Ok(json[" "].clone()), walker(&json, space_selector));
        assert_eq!(Ok(json[r#""#].clone()), walker(&json, empty_selector));
    }

    #[test]
    fn get_mix_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let mix_selector = r#""mix".[0]."first""#;
        assert_eq!(
            Ok(json["mix"][0]["first"].clone()),
            walker(&json, mix_selector)
        );
    }

    #[test]
    fn get_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[2:5]"#;
        assert_eq!(Ok(json!([3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_range_with_no_start() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[:5]"#;
        assert_eq!(Ok(json!([1, 2, 3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_range_with_no_end() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[2:]"#;
        assert_eq!(Ok(json!([3, 4, 5, 6, 7])), walker(&json, selector));
    }

    #[test]
    fn get_one_item_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[2:2]"#;
        assert_eq!(Ok(json!([3])), walker(&json, selector));
    }

    #[test]
    fn get_reversed_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[5:2]"#;
        assert_eq!(Ok(json!([6, 5, 4, 3])), walker(&json, selector));
    }

    #[test]
    fn get_original_from_reversed_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[5:2].[3:0]"#;
        assert_eq!(Ok(json!([3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[6:7]"#;
        assert_eq!(
            Err(String::from(
                r#"Range [6:7] is out of bound, node "range" has a length of 7"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_range_on_non_array_root() {
        let json: Value = serde_json::from_str(SINGLE_VALUE_DATA).unwrap();
        let selector = "[2:0]";
        assert_eq!(
            Err(String::from("Root element is not an array")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_range_on_non_array_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".[0:1]"#;
        assert_eq!(
            Err(String::from(r#"Node "nested" is not an array"#)),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_multi_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array","number""#;
        assert_eq!(
            Ok(json!([json["array"], json["number"]])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_multi_selection_with_space() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array",,, "#;
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_multi_selection_with_empty() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""array",,,"#;
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_multi_selection_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""range".[5:3],"array".[2:1]"#;
        assert_eq!(Ok(json!([[6, 5, 4], [3, 2]])), walker(&json, selector));
    }

    #[test]
    fn get_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""filter"|"color""#;
        assert_eq!(Ok(json!(["red", "green", "blue"])), walker(&json, selector));
    }

    #[test]
    fn get_double_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|"laptop"|"brand""#;
        assert_eq!(Ok(json!(["Apple", "Asus"])), walker(&json, selector));
    }

    #[test]
    fn get_wrong_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""filter"|"colors""#;
        assert_eq!(
            Err(String::from(
                r#"Node "colors" not found on the parent element"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_filter_with_no_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#"|"color""#;
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_filter_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""filter".[1:2]|"color""#;
        assert_eq!(Ok(json!(["green", "blue"])), walker(&json, selector));
    }

    #[test]
    fn get_wrong_filter_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""filter".[1:2]|"colors""#;
        assert_eq!(
            Err(String::from(
                r#"Node "colors" not found on the parent element"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_filter_with_multi_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""filter".[1:2]|"color","filter".[2:1]|"color""#;
        assert_eq!(
            Ok(json!([["green", "blue"], ["blue", "green"]])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_nested_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|"laptop"."brand""#;
        assert_eq!(Ok(json!(["Apple", "Asus"])), walker(&json, selector));
    }

    #[test]
    fn get_nested_filter_with_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|"laptop"."options".[0]"#;
        assert_eq!(Ok(json!(["a", "d"])), walker(&json, selector));
    }

    #[test]
    fn get_nested_filter_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|"laptop"."options".[1:2]"#;
        assert_eq!(Ok(json!([["b", "c"], ["e", "f"]])), walker(&json, selector));
    }

    #[test]
    fn get_filter_on_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested"|"some""#;
        assert_eq!(
            Err(String::from("A filter can only be applied to an array")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_flattened_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#".."filter-to-flatten""#;
        assert_eq!(
            Ok(json!(["c", "a", "c", "g", "a", "t"])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_flattened_groups() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#".."filter-to-flatten",.."filter-to-flatten""#;
        assert_eq!(
            Ok(json!([
                ["c", "a", "c", "g", "a", "t"],
                ["c", "a", "c", "g", "a", "t"]
            ])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_flattened_and_filtered_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#".."nested-filter-to-flatten"|"fruit"."dna""#;
        assert_eq!(
            Ok(json!([
                "c", "g", "a", "t", "c", "a", "c", "g", "t", "a", "t"
            ])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_properties_root() {
        let json: Value = serde_json::from_str(OBJECT_DATA).unwrap();
        let selector = r#"{"a","b"}"#;
        assert_eq!(Ok(json!({ "a": 7, "b": 11 })), walker(&json, selector));
    }

    #[test]
    fn get_unordered_properties_root() {
        let json: Value = serde_json::from_str(OBJECT_DATA).unwrap();
        let selector = r#"{"b","a"}"#;
        assert_eq!(Ok(json!({ "b": 11, "a": 7 })), walker(&json, selector));
    }

    #[test]
    fn get_properties_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{"a","b"}"#;
        assert_eq!(
            Ok(json!({ "a": "one", "b": "two" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_unordered_properties_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{"b","a"}"#;
        assert_eq!(
            Ok(json!({ "b": "two", "a": "one" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_properties_root() {
        let json: Value = serde_json::from_str(OBJECT_DATA).unwrap();
        let selector = r#"{"x","b"}"#;
        assert_eq!(
            Err(String::from(r#"Node "x" not found on the parent element"#)),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_properties_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{"x","b"}"#;
        assert_eq!(
            Err(String::from(
                r#"Node "x" not found on parent node "nested""#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_properties_in_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|"laptop"|{"price","brand"}"#;
        assert_eq!(
            Ok(json!([{"price": 9999, "brand": "Apple"}, {"price": 999, "brand": "Asus"}])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[]}"#;
        assert_eq!(
            Ok(json!({ "a": "one", "b": "two", "c": "three" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{"b",[0,2]}"#;
        assert_eq!(
            Ok(json!({ "b": "two", "0": "one", "2": "three" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_out_of_bound_index() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{"b",[0,3]}"#;
        assert_eq!(
            Err(String::from(
                r#"Index [3] is out of bound, node "nested" contains 3 properties"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[1:2]}"#;
        assert_eq!(
            Ok(json!({ "1": "two", "2": "three" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_range_reverse() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[2:1]}"#;
        assert_eq!(
            Ok(json!({ "2": "three", "1": "two" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_range_with_no_start() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[:2]}"#;
        assert_eq!(
            Ok(json!({ "0": "one", "1": "two", "2": "three" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_range_with_no_end() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[1:]}"#;
        assert_eq!(
            Ok(json!({ "1": "two", "2": "three" })),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_out_of_bound_start_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[4:9]}"#;
        assert_eq!(
            Err(String::from(
                r#"Range [4:9] is out of bound, node "nested" contains 3 properties"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_out_of_bound_end_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested".{[1:9]}"#;
        assert_eq!(
            Err(String::from(
                r#"Range [1:9] is out of bound, node "nested" contains 3 properties"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_out_of_bound_index_in_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|{[1,3]}"#;
        assert_eq!(
            Err(String::from(
                r#"Index [3] is out of bound, object contains 1 property"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_property_as_out_of_bound_range_in_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter"|{[1:3]}"#;
        assert_eq!(
            Err(String::from(
                r#"Range [1:3] is out of bound, object contains 1 property"#
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn check_whitespace() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let space_selector = r#"" ""#;
        let selector_with_spaces = r#""nested" .   "a""#;
        assert_eq!(
            Ok(json!("Yup, this too 🐼!")),
            walker(&json, space_selector)
        );
        assert_eq!(
            Ok(json["nested"]["a"].clone()),
            walker(&json, selector_with_spaces)
        );
    }

    #[test]
    fn check_truncate_on_root() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#".!"#;
        assert_eq!(
            Ok(json!({
                "array": [],
                "empty-array": [],
                "nested": {},
                "null": null,
                "number": 1337,
                "text": "some text",
                ".property..": "This is valid JSON!",
                "\"": "This is valid JSON as well",
                " ": "Yup, this too 🐼!",
                "": "Yup, again 🐨!",
                "mix": [],
                "range": [],
                "filter": [],
                "nested-filter": [],
                "filter-to-flatten": [],
                "nested-filter-to-flatten": [],
                "lenses": []
            })),
            walker(&json, selector)
        );
    }

    #[test]
    fn check_truncate_on_nested_value() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter".[0]."laptop"!"#;
        assert_eq!(
            Ok(json!({
                "brand": "Apple",
                "options": [],
                "price": 9999
            })),
            walker(&json, selector)
        );
    }

    #[test]
    fn check_truncate_on_groups() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter".[0]."laptop"!, "filter"!"#;
        assert_eq!(
            Ok(json!([
                {"brand": "Apple", "options": [], "price": 9999 },
                [{},{},{}]
            ])),
            walker(&json, selector)
        );
    }

    #[test]
    fn check_truncate_with_filter() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector = r#""nested-filter-to-flatten"|"fruit"!"#;
        assert_eq!(Ok(json!([{}, {}])), walker(&json, selector));
    }

    #[test]
    fn check_lenses() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector_keys_only = r#""lenses"|={"delta","alpha"}"#;
        let selector_key_value_number = r#""lenses"|={"delta":"4"}"#;
        let selector_key_value_null = r#""lenses"|={"beta":"null"}"#;
        let selector_key_value_string = r#""lenses"|={"delta":"something"}"#;
        let selector_key_value_multiple =
            r#""lenses"|={"delta":"something", "delta":"4", "alpha"}"#;
        assert_eq!(
            Ok(json!([
                {"alpha": 1, "beta": Value::Null},
                {"gamma": 3, "delta": "something"},
                {"alpha": 7},
                {"delta": 4}
            ])),
            walker(&json, selector_keys_only)
        );
        assert_eq!(
            Ok(json!([{"delta": 4}])),
            walker(&json, selector_key_value_number)
        );
        assert_eq!(
            Ok(json!([
                {"alpha": 1, "beta": Value::Null},
            ])),
            walker(&json, selector_key_value_null)
        );
        assert_eq!(
            Ok(json!([
                {"gamma": 3, "delta": "something"},
            ])),
            walker(&json, selector_key_value_string)
        );
        assert_eq!(
            Ok(json!([
                {"alpha": 1, "beta": Value::Null},
                {"gamma": 3, "delta": "something"},
                {"alpha": 7},
                {"delta": 4}
            ])),
            walker(&json, selector_key_value_multiple)
        );
    }
}
