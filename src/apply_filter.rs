use get_selection::get_selection;
use serde_json::json;
use serde_json::Value;
use types::{Selection, Selector};

/// Apply the filter selectors to a JSON value and returns a selection.
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
                        .map(|selection| selection
                            .clone()
                            .unwrap()
                            .last()
                            .unwrap()
                            .clone()).collect::<Vec<Value>>()
                )]),
            }
        }
        // Not an array, return the raw JSON content if there's no filter or
        // throw an error.
        None => match filter_selectors {
            Some(_) => {
                Err(String::from("A filter can only be applied to an array"))
            }
            None => Ok(vec![json.clone()]),
        },
    }
}
