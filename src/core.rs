extern crate regex;
extern crate serde_json;

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use serde_json::Value;
use types::{Selection, Selector, Selectors};
use utils::display_node_or_range;

/// Get the trimmed text of the match with a default of an empty
/// string if the group didn't participate in the match.
fn get_selector(capture: &str) -> Selector {
    let capture = capture.trim();

    if capture.starts_with('\"') {
        // let cap_string = String::from(cap);
        // Drop the enclosing double quotes in this case.
        // let inner_cap = cap_string[1..cap_string.len() - 1];
        Selector::Default(String::from(&capture[1..capture.len() - 1]))
    } else {
        // Array range, e.g. 0:3.
        lazy_static! {
            static ref RANGE_REGEX: Regex = Regex::new(r"(\d+):(\d+)").unwrap();
        }

        let ranges: Vec<(&str, &str)> = RANGE_REGEX
            .captures_iter(capture)
            .map(|capture| {
                (
                    capture.get(1).map_or("", |m| m.as_str()),
                    capture.get(2).map_or("", |m| m.as_str()),
                )
            }).collect();
        if ranges.is_empty() {
            // Returns the initial captured value.
            Selector::Default(String::from(capture))
        } else {
            // Returns the range as a tuple of the form (start,end).
            let (start, end) = &ranges[0];
            Selector::Range((
                usize::from_str_radix(start, 10).unwrap(),
                usize::from_str_radix(end, 10).unwrap(),
            ))
        }
    }
}

/// Walks through a JSON array.
pub fn array_walker(
    map_index: usize,
    array_index: isize,
    inner_json: &Value,
    raw_selector: &str,
    selectors: &Selectors,
) -> Result<Value, String> {
    // A Negative index has been provided.
    if (array_index).is_negative() {
        return Err(String::from("Invalid negative array index"));
    }

    // A JSON null value has been found (array).
    if inner_json[array_index as usize] == Value::Null {
        let error_message = match inner_json.as_array() {
            // Trying to access an out of bound index on a node
            // or on the root element.
            Some(array) => {
                if selectors.len() == 1 {
                    [
                        "Index (",
                        raw_selector,
                        ") is out of bound, root element has a length of",
                        &(array.len()).to_string(),
                    ]
                        .join(" ")
                } else {
                    [
                        "Index (",
                        raw_selector,
                        ") is out of bound,",
                        &display_node_or_range(
                            &selectors[map_index - 1],
                            false,
                        ),
                        "has a length of",
                        &(array.len()).to_string(),
                    ]
                        .join(" ")
                }
            }
            // Trying to access an index on a node which is not
            // an array.
            None => {
                if selectors.len() == 1 {
                    ["Root element is not an array"].join(" ")
                } else {
                    [
                        &display_node_or_range(&selectors[map_index - 1], true),
                        "is not an array",
                    ]
                        .join(" ")
                }
            }
        };

        return Err(error_message);
    }

    // Match found.
    Ok(inner_json[array_index as usize].clone())
}

/// Returns a range selection or an error.
pub fn range_selector(
    map_index: usize,
    inner_json: &Value,
    start: usize,
    end: usize,
    selectors: &Selectors,
) -> Result<Value, String> {
    let is_default = start < end;

    // Check the range validity.
    if inner_json.as_array().unwrap().len() < start
        || inner_json.as_array().unwrap().len() < (end + 1)
    {
        return Err(if selectors.len() == 1 {
            [
                "Range (",
                start.to_string().as_str(),
                ":",
                end.to_string().as_str(),
                ") is out of bound, root element has a length of",
                &(inner_json.as_array().unwrap().len()).to_string(),
            ]
                .join(" ")
        } else {
            [
                "Range (",
                start.to_string().as_str(),
                ":",
                end.to_string().as_str(),
                ") is out of bound,",
                &display_node_or_range(&selectors[map_index - 1], false),
                "has a length of",
                &(inner_json.as_array().unwrap().len()).to_string(),
            ]
                .join(" ")
        });
    }

    Ok(if is_default {
        json!(inner_json.as_array().unwrap()[start..=end])
    } else {
        // Get the normalized slice selection, i.e. from end to start.
        let normalized_range_selection =
            json!(inner_json.as_array().unwrap()[end..=start]);
        // Reverse it.
        let reversed_range_selection: Vec<&Value> = normalized_range_selection
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .collect();
        json!(reversed_range_selection)
    })
}

/// Returns a selection based on selectors and some JSON content.
fn get_selection(selectors: &Selectors, json: &Value) -> Selection {
    // Local copy of the origin json that will be reused in the loop.
    let mut inner_json = json.clone();
    selectors
        .iter()
        .enumerate()
        .map(|(map_index, current_selector)| -> Result<Value, String> {
            match current_selector {
                // Default selector.
                Selector::Default(raw_selector) => {
                    // Array case.
                    if let Ok(array_index) = raw_selector.parse::<isize>() {
                        return match array_walker(
                            map_index,
                            array_index,
                            &inner_json.clone(),
                            raw_selector,
                            &selectors,
                        ) {
                            Ok(json) => {
                                inner_json = json.clone();
                                Ok(json.clone())
                            }
                            Err(error) => Err(error),
                        };
                    }

                    // A JSON null value has been found (non array).
                    if inner_json[raw_selector] == Value::Null {
                        if map_index == 0 {
                            Err([
                                "Node (",
                                raw_selector,
                                ") is not the root element",
                            ]
                                .join(" "))
                        } else {
                            Err([
                                "Node (",
                                raw_selector,
                                ") not found on parent",
                                &display_node_or_range(
                                    &selectors[map_index - 1],
                                    false,
                                ),
                            ]
                                .join(" "))
                        }
                    } else {
                        inner_json = inner_json[raw_selector].clone();
                        Ok(inner_json.clone())
                    }
                }
                // Range selector.
                Selector::Range((start, end)) => match range_selector(
                    map_index,
                    &inner_json.clone(),
                    *start,
                    *end,
                    &selectors,
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json.clone())
                    }
                    Err(error) => Err(error),
                },
            }
        }).collect()
}

/// Walks through a group.
fn group_walker(
    capture: &regex::Captures<'_>,
    filter: Option<&str>,
    json: &Value,
) -> Selection {
    lazy_static! {
        static ref SUB_GROUP_REGEX: Regex =
            Regex::new(r#"("[^"]+")|([^.]+)"#).unwrap();
    }

    let group = capture.get(0).map_or("", |m| m.as_str()).trim();

    println!("** {:?} **", filter);
    // Empty group, return early.
    if group.is_empty() {
        return Err(String::from("Empty group"));
    }

    // Capture sub-groups of double quoted selectors and simple ones surrounded
    // by dots on the group itself.
    let selectors: Vec<Selector> = SUB_GROUP_REGEX
        .captures_iter(group)
        .map(|capture| get_selector(capture.get(0).map_or("", |m| m.as_str())))
        .collect();

    // Perform the same operation on the filter.
    let filter_selectors = match filter {
        Some(filter) => Some(
            SUB_GROUP_REGEX
                .captures_iter(filter)
                .map(|capture| {
                    get_selector(capture.get(0).map_or("", |m| m.as_str()))
                }).collect::<Vec<Selector>>(),
        ),
        None => None,
    };

    // Returns a Result of values or an Err early on, stopping the iteration
    // as soon as the latter is encountered.
    let items: Selection = get_selection(&selectors, &json);
    // Check for empty selection, in this case we assume that the user expects
    // to get back the complete raw JSON back for this group.
    match items {
        Ok(items) => {
            if items.is_empty() {
                println!("=== {:?}", apply_filter(&json, &filter_selectors));
                apply_filter(&json, &filter_selectors)
            } else {
                Ok(items)
            }
        }
        Err(items) => Err(items),
    }
}

///
pub fn apply_filter(
    json: &Value,
    filter_selectors: &Option<Vec<Selector>>,
) -> Selection {
    // Apply the filter iff the provided JSON value is an array.
    match json.as_array() {
        Some(array) => {
            let selections: Vec<Selection> = array
                .iter()
                .cloned()
                .map(|partial_json| -> Selection {
                    match filter_selectors {
                        // Get the selection based on the filter.
                        Some(ref selectors) => {
                            get_selection(&selectors, &partial_json)
                        }
                        // No filter, return the JSON value.
                        None => Ok(vec![partial_json]),
                    }
                }).collect();
            // Try to find the first error.
            match selections
                .iter()
                .find_map(|selection| selection.clone().err())
            {
                // Throw it back.
                Some(error) => Err(error),
                // No error in this case, we can safely unwrap.
                None => Ok(vec![json!(
                    selections
                        .iter()
                        .map(|selection| selection.clone().unwrap())
                        .flatten()
                        .collect::<Vec<Value>>()
                )]),
            }
        }
        None => Ok(vec![json.clone()]),
    }
}

/// Given some selector walk over the JSON file.
pub fn walker(json: &Value, selector: Option<&str>) -> Result<Value, String> {
    // A Selector has been found.
    if let Some(selector) = selector {
        lazy_static! {
            static ref FILTER_REGEX: Regex =
                Regex::new(r"^(.*)\|([^|]+)$").unwrap();
            static ref GROUP_REGEX: Regex = Regex::new(r"([^,]+)").unwrap();
        }

        let selection_with_filter: Vec<(&str, &str)> = FILTER_REGEX
            .captures_iter(selector)
            .map(|capture| {
                (
                    capture.get(1).map_or("", |m| m.as_str()),
                    capture.get(2).map_or("", |m| m.as_str()),
                )
            }).collect();

        let selector_and_filter = if selection_with_filter.is_empty() {
            // No filter, use the initial selector.
            (selector, None)
        } else {
            // Use the left part before the filter.
            (selection_with_filter[0].0, Some(selection_with_filter[0].1))
        };

        // Capture groups separated by commas, apply the selector for the
        // current group and return a Result of values or an Err early on.
        let groups: Result<Vec<Value>, String> = GROUP_REGEX
            .captures_iter(selector_and_filter.0)
            .map(|capture| group_walker(&capture, selector_and_filter.1, json))
            .map(|s| -> Result<Value, String> {
                match s {
                    Ok(items) => Ok(items.last().unwrap().clone()),
                    Err(error) => Err(error.clone()),
                }
            }).collect();

        return match groups {
            Ok(groups) => match groups.len() {
                0 => Err(String::from("Empty selection")),
                // One group.
                1 => Ok(json!(groups[0])),
                // Multiple groups.
                _ => Ok(json!(groups)),
            },
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
        "\"": "This is valid JSON as well",
        " ": "Yup, this too 🐼!",
        "": "Yup, again 🐨!",
        "mix": [{ "first": 1 }],
        "range": [1, 2, 3, 4, 5, 6, 7]
    }"#;

    #[test]
    fn get_text() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("text");
        assert_eq!(Ok(json["text"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_number() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("number");
        assert_eq!(Ok(json["number"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array");
        assert_eq!(Ok(json["array"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.0");
        assert_eq!(Ok(json["array"][0].clone()), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_item_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.3");
        assert_eq!(
            Err(String::from(
                "Index ( 3 ) is out of bound, node ( array ) has a length of 3"
            )),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_out_of_bound_item_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector: Option<&str> = Some("3");
        assert_eq!(
            Err(String::from(
                "Index ( 3 ) is out of bound, root element has a length of 3"
            )),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_negative_index_in_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array.-1");
        assert_eq!(
            Err(String::from("Invalid negative array index")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_negative_index_in_root_array() {
        let json_array: Value = serde_json::from_str(ARRAY_DATA).unwrap();
        let array_selector: Option<&str> = Some("-1");
        assert_eq!(
            Err(String::from("Invalid negative array index")),
            walker(&json_array, array_selector)
        );
    }

    #[test]
    fn get_index_in_non_array() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("text.1");
        let root_selector: Option<&str> = Some("1");
        assert_eq!(
            Err(String::from("Node ( text ) is not an array")),
            walker(&json, selector)
        );
        assert_eq!(
            Err(String::from("Root element is not an array")),
            walker(&json, root_selector)
        );
    }

    #[test]
    fn get_non_existing_root_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("foo");
        assert_eq!(
            Err(String::from("Node ( foo ) is not the root element")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_non_existing_child_node() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("nested.d");
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
        let selector: Option<&str> = Some("nested.a");
        assert_eq!(Ok(json["nested"]["a"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_single_value() {
        let json_single_value: Value =
            serde_json::from_str(SINGLE_VALUE_DATA).unwrap();
        let selector: Option<&str> = Some(".");
        assert_eq!(
            Ok(json_single_value.clone()),
            walker(&json_single_value, selector)
        );
    }

    #[test]
    fn get_empty() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("");
        assert_eq!(
            Err(String::from("Empty selection")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_raw_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some(".");
        assert_eq!(Ok(json.clone()), walker(&json, selector));
    }

    #[test]
    fn get_weird_json() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let dot_selector: Option<&str> = Some(r#"".property..""#);
        let quote_selector: Option<&str> = Some(r#"""""#);
        let space_selector: Option<&str> = Some(r#"" ""#);
        let empty_selector: Option<&str> = Some(r#""""#);
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
        let mix_selector: Option<&str> = Some("mix.0.first");
        assert_eq!(
            Ok(json["mix"][0]["first"].clone()),
            walker(&json, mix_selector)
        );
    }

    #[test]
    fn get_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("range.2:5");
        assert_eq!(Ok(json!([3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_one_item_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("range.2:2");
        assert_eq!(Ok(json!([3])), walker(&json, selector));
    }

    #[test]
    fn get_reversed_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("range.5:2");
        assert_eq!(Ok(json!([6, 5, 4, 3])), walker(&json, selector));
    }

    #[test]
    fn get_original_from_reversed_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("range.5:2.3:0");
        assert_eq!(Ok(json!([3, 4, 5, 6])), walker(&json, selector));
    }

    #[test]
    fn get_out_of_bound_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("range.6:7");
        assert_eq!(
            Err(String::from("Range ( 6 : 7 ) is out of bound, node ( range ) has a length of 7")),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_multi_selection() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array,number");
        assert_eq!(
            Ok(json!([json["array"], json["number"]])),
            walker(&json, selector)
        );
    }

    #[test]
    fn get_multi_selection_with_space() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array,,, ");
        assert_eq!(Err(String::from("Empty group")), walker(&json, selector));
    }

    #[test]
    fn get_multi_selection_with_empty() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("array,,,");
        assert_eq!(Ok(json["array"].clone()), walker(&json, selector));
    }

    #[test]
    fn get_multi_selection_with_range() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        let selector: Option<&str> = Some("range.5:3,array.2:1");
        assert_eq!(Ok(json!([[6, 5, 4], [3, 2]])), walker(&json, selector));
    }
}
