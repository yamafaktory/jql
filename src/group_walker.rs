use apply_filter::apply_filter;
use flatten_json_array::flatten_json_array;
use get_selection::get_selection;
use get_selector::get_selector;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use serde_json::Value;
use types::{Group, MaybeArray, Selection, Selector};

/// Walks through a group.
pub fn group_walker(
    (spread, selectors, filters): Group,
    json: &Value,
) -> Result<Value, String> {
    lazy_static! {
        static ref FILTER_REGEX: Regex =
            Regex::new(r"^(\.{2})?(.*)(?:\|([^|]*))+$").unwrap();
        static ref SUB_GROUP_REGEX: Regex =
            Regex::new(r#"("[^"]+")|([^.]+)"#).unwrap();
    }

    // let parsed_group: (Option<()>, &str, Option<&str>) = FILTER_REGEX
    //     .captures_iter(group)
    //     .map(|capture| {
    //         println!("--++ {:?}", capture);
    //         (
    //             // Spread capture.
    //             capture.get(1).and_then(|_| Some(())),
    //             // Group capture.
    //             capture.get(2).map_or("", |m| m.as_str()),
    //             // Filter capture.
    //             capture.get(3).and_then(|m| Some(m.as_str())),
    //         )
    //     })
    //     .nth(0)
    //     // If nothing is captured, use the initial group.
    //     .unwrap_or_else(|| (None, group, None));

    // Empty group, return early.
    if selectors.is_empty() {
        return Err(String::from("Empty group"));
    }

    let parsed_selectors: Vec<Selector> = selectors
        .iter()
        .map(|selector| get_selector(selector))
        .collect();

    // Perform the same operation on the filter.
    let filter_selectors: Vec<Selector> = filters
        .iter()
        .map(|selector| get_selector(selector))
        .collect();

    // Returns a Result of values or an Err early on, stopping the iteration
    // as soon as the latter is encountered.
    let items: Selection = get_selection(&parsed_selectors, &json);

    match items {
        Ok(ref items) => {
            // Check for an empty selection, in this case we assume that the user
            // expects to get back the complete raw JSON for this group.
            let output_json = if items.is_empty() {
                json.clone()
            } else {
                json!(items.last()).clone()
            };

            let is_spreading = spread.is_some();

            match apply_filter(&output_json, &filter_selectors) {
                Ok(filtered) => match filtered {
                    MaybeArray::Array(array) => Ok(if is_spreading {
                        flatten_json_array(&json!(array))
                    } else {
                        json!(array)
                    }),
                    MaybeArray::NonArray(single_value) => {
                        if is_spreading {
                            Err(String::from("Only arrays can be flattened"))
                        } else {
                            // We know that we are holding a single value
                            // wrapped inside a MaybeArray::NonArray enum.
                            // We need to pick the first item of the vector.
                            Ok(json!(single_value[0]))
                        }
                    }
                },
                Err(error) => Err(error),
            }
        }
        Err(items) => Err(items),
    }
}
