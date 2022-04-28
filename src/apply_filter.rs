use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    get_selection::get_selection,
    types::{ExtendedSelections, InnerObject, MaybeArray, Selections, Selector},
};

/// Gets the lenses from the filter lenses.
fn get_lenses(filter_lenses: &[Selector]) -> Vec<(&str, Option<&str>)> {
    filter_lenses
        .iter()
        .filter_map(|selector| match selector {
            Selector::Object(inner_objects) => Some(
                inner_objects
                    .par_iter()
                    .fold_with(Vec::new(), |mut acc, inner_object| {
                        if let InnerObject::KeyValue(key, value) = inner_object {
                            acc.push((key.as_str(), value.as_deref()));
                        }

                        acc
                    })
                    .flatten()
                    .collect::<Vec<(&str, Option<&str>)>>(),
            ),
            _ => None,
        })
        .flatten()
        .collect::<Vec<(&str, Option<&str>)>>()
}

/// Check if a given key/value pair matches some lenses.
fn match_lenses(lenses: &[(&str, Option<&str>)], (key, value): (&String, &Value)) -> bool {
    lenses.iter().any(|(lens_key, lens_value)| {
        match *lens_value {
            // Both key and value.
            Some(lens_value) => {
                key == lens_key
                    && match value {
                        Value::String(string) => lens_value == string,
                        Value::Number(number) => lens_value == number.to_string(),
                        Value::Null => lens_value == "null",
                        // We don't want to perform any other comparison for
                        // other primitives.
                        _ => false,
                    }
            }
            // Based on the key only.
            None => key == lens_key,
        }
    })
}

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
                .filter(|partial_json| {
                    // Check whether we have some lenses or not and if this is
                    // an object since lenses can only be applied to objects.
                    if !lenses.is_empty() && partial_json.is_object() {
                        // We can safely unwrap here based on the conditional
                        // above.
                        let object = partial_json.as_object().unwrap();

                        object
                            .iter()
                            .any(|key_value| match_lenses(&lenses, key_value))
                    } else {
                        true
                    }
                })
                .map(|partial_json| -> Selections {
                    if filter_selectors.is_empty() {
                        Ok(vec![partial_json])
                    } else {
                        get_selection(filter_selectors, &partial_json)
                    }
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
