use crate::array_walker::array_walker;
use crate::range_selector::range_selector;
use crate::types::{Display, InnerObject, Selection, Selections, Selector, Selectors};

use rayon::prelude::*;
use serde_json::{json, Map, Value};

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

fn object_to_vec(inner_json: &Value) -> Vec<(String, Value)> {
    // Make a mutable copy of the inner JSON.
    let mut inner_json_mut = inner_json.clone();

    inner_json_mut
        .as_object_mut()
        .unwrap()
        .to_owned()
        .into_iter()
        .collect::<Vec<(String, Value)>>()
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
                                match property {
                                    InnerObject::Index(indexes) => {
                                        let key_and_values = object_to_vec(&inner_json);
                                        let last_index = key_and_values.len() - 1;

                                        match indexes.par_iter().find_last(|&&x| x > last_index) {
                                            Some(index) => {
                                                return Err([
                                                    "Index [",
                                                    index.to_string().as_str(),
                                                    "] is out of bound, ",
                                                    &selectors[map_index - 1].as_str(false),
                                                    " contains ",
                                                    &(key_and_values.len()).to_string(),
                                                    " properties",
                                                ]
                                                .join(""));
                                            }
                                            None => {
                                                let map = indexes.iter().fold(
                                                    Map::with_capacity(indexes.len()),
                                                    |mut acc, index| {
                                                        acc.insert(
                                                            index.to_string(),
                                                            key_and_values[*index].1.clone(),
                                                        );

                                                        acc
                                                    },
                                                );

                                                Ok(json!(map))
                                            }
                                        }
                                    }
                                    InnerObject::Key(key) => {
                                        match apply_selector(&inner_json, map_index, key, selectors)
                                        {
                                            Ok(value) => match acc {
                                                Ok(mut current) => {
                                                    // Get the associated mutable Map and insert
                                                    // the property.
                                                    current
                                                        .as_object_mut()
                                                        .unwrap()
                                                        .insert(key.clone(), value);
                                                    Ok(current)
                                                }
                                                Err(error) => Err(error),
                                            },
                                            Err(error) => Err(error),
                                        }
                                    }
                                    // This selector is pretty dumb but is used as a guard
                                    // if an empty array is provided.
                                    InnerObject::Array => Ok(inner_json.clone()),
                                    InnerObject::Range((start, end)) => {
                                        let key_and_values = object_to_vec(&inner_json);
                                        let last_index = key_and_values.len() - 1;
                                        let start_with_default = start.unwrap_or(0);
                                        let end_with_default = end.unwrap_or(last_index);
                                        let is_default = start_with_default < end_with_default;

                                        // Safe out of bound checks.
                                        if start_with_default > last_index
                                            || end_with_default > last_index
                                        {
                                            return Err([
                                                "Range [",
                                                start_with_default.to_string().as_str(),
                                                ":",
                                                end_with_default.to_string().as_str(),
                                                "] is out of bound, ",
                                                &selectors[map_index - 1].as_str(false),
                                                " contains ",
                                                &(key_and_values.len()).to_string(),
                                                " properties",
                                            ]
                                            .join(""));
                                        }

                                        let indexes = if is_default {
                                            (start_with_default..=end_with_default)
                                                .step_by(1)
                                                .collect::<Vec<usize>>()
                                        } else {
                                            (end_with_default..=start_with_default)
                                                .step_by(1)
                                                .collect::<Vec<usize>>()
                                                .into_par_iter()
                                                .rev()
                                                .collect::<Vec<usize>>()
                                        };

                                        let map = indexes.iter().fold(
                                            Map::with_capacity(indexes.len()),
                                            |mut acc, index| {
                                                acc.insert(
                                                    index.to_string(),
                                                    key_and_values[*index].1.clone(),
                                                );

                                                acc
                                            },
                                        );

                                        Ok(json!(map))
                                    }
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
                    selectors,
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
                    selectors,
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
                    match array_walker(array_indexes, &inner_json, map_index, selectors) {
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
