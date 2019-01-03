use crate::array_walker::array_walker;
use crate::range_selector::range_selector;
use crate::types::{Display, Selection, Selector, Selectors};
use serde_json::{json, Value};

fn apply_selector(
    inner_json: &Value,
    map_index: usize,
    raw_selector: &str,
    selectors: &Selectors,
) -> Result<Value, String> {
    // No JSON value has been found.
    if inner_json.get(raw_selector).is_none() {
        if map_index == 0 {
            Err([
                r#"Node ""#,
                raw_selector,
                r#"" not found on the parent element"#,
            ]
            .join(""))
        } else {
            Err([
                r#"Node ""#,
                raw_selector,
                r#"" not found on parent "#,
                &selectors[map_index - 1].as_str(false),
            ]
            .join(""))
        }
    // Default case.
    } else {
        Ok(inner_json[raw_selector].clone())
    }
}

/// Merge two JSON objects.
/// https://github.com/serde-rs/json/issues/377#issuecomment-341490464
fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (key, value) in b {
                merge(a.entry(key.clone()).or_insert(Value::Null), value);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

/// Returns a selection based on selectors and a JSON content as a Result of
/// values or an Err early on, stopping the iteration as soon as the latter is
/// encountered.
pub fn get_selection(selectors: &Selectors, json: &Value) -> Selection {
    // Local copy of the original JSON that will be reused in the loop.
    let mut inner_json = json.clone();

    selectors
        .iter()
        .enumerate()
        .map(|(map_index, current_selector)| -> Result<Value, String> {
            match current_selector {
                // Object selector.
                Selector::Object(properties) => properties.iter().fold(
                    Ok(Value::Null),
                    |acc: Result<Value, String>, property| {
                        let value = apply_selector(
                            &inner_json,
                            map_index,
                            property,
                            selectors,
                        );
                        match value {
                            Ok(value) => match acc {
                                Ok(mut current) => {
                                    merge(
                                        &mut current,
                                        &json!({ property.as_str(): value }),
                                    );
                                    Ok(current)
                                }
                                Err(error) => Err(error),
                            },
                            Err(error) => Err(error),
                        }
                    },
                ),

                // Default selector.
                Selector::Default(raw_selector) => {
                    match apply_selector(
                        &inner_json,
                        map_index,
                        raw_selector,
                        selectors,
                    ) {
                        Ok(ref json) => {
                            inner_json = json.clone();
                            Ok(json.clone())
                        }
                        Err(error) => Err(error),
                    }
                }

                // Range selector.
                Selector::Range((start, end)) => match range_selector(
                    map_index,
                    &inner_json.clone(),
                    *start,
                    *end,
                    &selectors,
                    if map_index == 0 {
                        None
                    } else {
                        Some(&selectors[map_index - 1])
                    },
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json.clone())
                    }
                    Err(error) => Err(error),
                },

                // Array selector.
                Selector::Array => match range_selector(
                    map_index,
                    &inner_json.clone(),
                    Some(0),
                    None,
                    &selectors,
                    if map_index == 0 {
                        None
                    } else {
                        Some(&selectors[map_index - 1])
                    },
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json.clone())
                    }
                    Err(error) => Err(error),
                },

                // Index selector.
                Selector::Index(array_indexes) => match array_walker(
                    map_index,
                    &array_indexes,
                    &inner_json,
                    &selectors,
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json.clone())
                    }
                    Err(error) => Err(error),
                },
            }
        })
        .collect()
}
