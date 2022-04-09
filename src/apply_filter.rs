use crate::{
    get_selection::get_selection,
    types::{ExtendedSelections, InnerObject, MaybeArray, Selections, Selector},
};

use rayon::prelude::*;
use serde_json::{json, Map, Value};

/// Gets the lenses from the filter lenses.
fn get_lenses(filter_lenses: &[Selector]) -> Vec<String> {
    filter_lenses
        .iter()
        .filter_map(|selector| match selector {
            Selector::Object(inner_objects) => Some(
                inner_objects
                    .par_iter()
                    .fold_with(Vec::new(), |mut acc, inner_object| {
                        if let InnerObject::Key(key) = inner_object {
                            acc.push(key.to_owned());
                        }

                        acc
                    })
                    .flatten()
                    .collect::<Vec<String>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect::<Vec<String>>()
}

/// Apply the filter selectors to a JSON value and returns a selection.
pub fn apply_filter(
    filter_selectors: &[Selector],
    filter_lenses: &[Selector],
    json: &Value,
) -> ExtendedSelections {
    // Apply the filter iff the provided JSON value is an array.
    match json.as_array() {
        Some(array) => {
            let lenses = get_lenses(filter_lenses);

            let selections: Vec<Selections> = array
                .par_iter()
                .cloned()
                .map(|partial_json| -> Selections {
                    // Check whether we have some lenses or not.
                    if !lenses.is_empty() {
                        // Lenses can only be applied to JSON objects.
                        if partial_json.is_object() {
                            let object = partial_json.as_object().unwrap();
                            let map = object.iter().fold(
                                Map::with_capacity(object.len()),
                                |mut acc, (key, value)| {
                                    // Push to the map if we have a matching
                                    // lens.
                                    if lenses.iter().any(|lens| lens == key) {
                                        acc.insert(key.to_string(), value.to_owned());
                                    }

                                    acc
                                },
                            );

                            // Avoid returning an empty map if no match has
                            // been found.
                            if map.is_empty() {
                                return Ok(vec![]);
                            }

                            return Ok(vec![json!(map)]);
                        }

                        return Ok(vec![partial_json]);
                    }

                    if filter_selectors.is_empty() {
                        return Ok(vec![partial_json]);
                    }

                    get_selection(filter_selectors, &partial_json)
                })
                .collect();

            // Try to find the first error.
            match selections
                .iter()
                .find_map(|selection| selection.as_ref().err())
            {
                // Throw it back.
                Some(error) => Err(error.to_string()),
                // No error in this case, proceed.
                None => Ok(MaybeArray::Array(selections.iter().fold(
                    Vec::with_capacity(selections.len()),
                    |mut acc: Vec<Value>, selection| {
                        if let Ok(values) = selection {
                            if !values.is_empty() {
                                acc.push(json!(values.last().unwrap()));
                            }
                        }

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
