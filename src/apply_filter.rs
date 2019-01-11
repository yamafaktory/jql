use crate::get_selection::get_selection;
use crate::types::{ExtendedSelections, MaybeArray, Selections, Selector};
use serde_json::json;
use serde_json::Value;
use rayon::prelude::*;

/// Apply the filter selectors to a JSON value and returns a selection.
pub fn apply_filter(
    filter_selectors: &[Selector],
    json: &Value,
) -> ExtendedSelections {
    // Apply the filter iff the provided JSON value is an array.
    match json.as_array() {
        Some(array) => {
            let selections: Vec<Selections> = array
                .par_iter()
                .cloned()
                .map(|partial_json| -> Selections {
                    if filter_selectors.is_empty() {
                        Ok(vec![partial_json])
                    } else {
                        get_selection(&filter_selectors, &partial_json)
                    }
                })
                .collect();

            // Try to find the first error.
            match selections
                .iter()
                .find_map(|selection| selection.clone().err())
            {
                // Throw it back.
                Some(error) => Err(error),
                // No error in this case, we can safely unwrap.
                None => Ok(MaybeArray::Array(selections.iter().fold(
                    Vec::new(),
                    |mut acc: Vec<Value>, selection| {
                        acc.push(json!(selection
                            .clone()
                            .unwrap()
                            .last()
                            .unwrap()
                            .clone()));

                        acc
                    },
                ))),
            }
        }
        // Not an array, return the raw JSON content if there's no filter or
        // throw an error.
        None => {
            if filter_selectors.is_empty() {
                Ok(MaybeArray::NonArray(vec![json.clone()]))
            } else {
                Err(String::from("A filter can only be applied to an array"))
            }
        }
    }
}
