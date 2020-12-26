use crate::array_walker::array_walker;
use crate::range_selector::range_selector;
use crate::types::{Display, Selection, Selections, Selector, Selectors};

use rayon::prelude::*;
use serde_json::{json, Value};

fn apply_selector(
    inner_json: &Value,
    map_index: usize,
    raw_selector: &str,
    selectors: &Selectors,
) -> Selection {
    // No JSON value has been found.
    if inner_json.get(raw_selector).is_none() {
        if map_index == 0 {
            return Err([
                r#"Node ""#,
                raw_selector,
                r#"" not found on the parent element"#,
            ]
            .join(""));
        } else {
            return Err([
                r#"Node ""#,
                raw_selector,
                r#"" not found on parent "#,
                &selectors[map_index - 1].as_str(false),
            ]
            .join(""));
        }
    }

    // Default case.
    Ok(inner_json[raw_selector].clone())
}

/// Returns a selection based on selectors and a JSON content as a Result of
/// values or an Err early on, stopping the iteration as soon as the latter is
/// encountered.
pub fn get_selection(selectors: &Selectors, json: &Value) -> Selections {
    // Local copy of the original JSON that will be reused in the loop.
    let mut inner_json = json.clone();

    selectors
        .iter()
        .enumerate()
        .map(|(map_index, current_selector)| -> Selection {
            match current_selector {
                // Object selector.
                Selector::Object(properties) => {
                    properties
                        .par_iter()
                        .fold(
                            || Ok(json!({})),
                            |acc: Selection, property| {
                                match apply_selector(&inner_json, map_index, property, selectors) {
                                    Ok(value) => match acc {
                                        Ok(mut current) => {
                                            // Get the associated mutable Map and insert
                                            // the property.
                                            current
                                                .as_object_mut()
                                                .unwrap()
                                                .insert(property.clone(), value);
                                            Ok(current)
                                        }
                                        Err(error) => Err(error),
                                    },
                                    Err(error) => Err(error),
                                }
                            },
                        )
                        .reduce(
                            || Ok(json!({})),
                            |first, second| {
                                first.and_then(|mut first| {
                                    second.map(|mut second| {
                                        first
                                            .as_object_mut()
                                            .unwrap()
                                            .extend(second.as_object_mut().unwrap().clone());

                                        first
                                    })
                                })
                            },
                        )
                }

                // Default selector.
                Selector::Default(raw_selector) => {
                    match apply_selector(&inner_json, map_index, raw_selector, selectors) {
                        Ok(ref json) => {
                            inner_json = json.clone();
                            Ok(json.clone())
                        }
                        Err(error) => Err(error),
                    }
                }

                // Range selector.
                Selector::Range((start, end)) => match range_selector(
                    *end,
                    &inner_json.clone(),
                    map_index,
                    if map_index == 0 {
                        None
                    } else {
                        Some(&selectors[map_index - 1])
                    },
                    &selectors,
                    *start,
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json)
                    }
                    Err(error) => Err(error),
                },

                // Array selector.
                Selector::Array => match range_selector(
                    None,
                    &inner_json.clone(),
                    map_index,
                    if map_index == 0 {
                        None
                    } else {
                        Some(&selectors[map_index - 1])
                    },
                    &selectors,
                    Some(0),
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json)
                    }
                    Err(error) => Err(error),
                },

                // Index selector.
                Selector::Index(array_indexes) => {
                    match array_walker(&array_indexes, &inner_json, map_index, &selectors) {
                        Ok(json) => {
                            inner_json = json.clone();
                            Ok(json)
                        }
                        Err(error) => Err(error),
                    }
                }
            }
        })
        .collect()
}
