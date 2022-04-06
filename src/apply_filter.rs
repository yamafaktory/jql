use crate::{
    get_selection::get_selection,
    types::{ExtendedSelections, InnerObject, MaybeArray, Selections, Selector},
};

use rayon::prelude::*;
use serde_json::{json, Value};

/// Apply the filter selectors to a JSON value and returns a selection.
pub fn apply_filter(
    filter_selectors: &[Selector],
    filter_lenses: &[Selector],
    json: &Value,
) -> ExtendedSelections {
    // Apply the filter iff the provided JSON value is an array.
    match json.as_array() {
        Some(array) => {
            let lenses = filter_lenses
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
                .collect::<Vec<String>>();

            dbg!(lenses);

            let selections: Vec<Selections> = array
                .par_iter()
                .cloned()
                .map(|partial_json| -> Selections {
                    if !filter_lenses.is_empty() {
                        //
                        //
                        if partial_json.is_object() {
                            // let t = partial_json
                            //     .as_object()
                            //     .unwrap()
                            //     .iter()
                            //     .filter_map(|(key, value)| {
                            //         // if filter_lenses
                            //         //     .iter()
                            //         //     .any(|current_index| index == current_index)
                            //         // {
                            //         //     return None;
                            //         // }
                            //
                            //         None
                            //     })
                            //     .collect();

                            return Ok(vec![partial_json]);
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
                .find_map(|selection| selection.clone().err())
            {
                // Throw it back.
                Some(error) => Err(error),
                // No error in this case, we can safely unwrap.
                None => Ok(MaybeArray::Array(selections.iter().fold(
                    Vec::new(),
                    |mut acc: Vec<Value>, selection| {
                        acc.push(json!(selection.clone().unwrap().last().unwrap().clone()));

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
