use rayon::prelude::*;
use serde_json::{json, Value};

use crate::{
    get_selection::get_selection,
    types::{ExtendedSelections, Filter, InnerObject, MaybeArray, Selections, Selector},
};

type KeyValueTuple<'a> = (&'a str, Option<&'a str>);

/// Gets the lenses from the filter lens.
fn get_lenses(filter_lens: &Selector) -> Vec<KeyValueTuple> {
    match filter_lens {
        Selector::Object(inner_objects) => inner_objects
            .par_iter()
            .fold_with(Vec::new(), |mut acc, inner_object| {
                if let InnerObject::KeyValue(key, value) = inner_object {
                    acc.push((key.as_str(), value.as_deref()));
                }

                acc
            })
            .flatten()
            .collect::<Vec<KeyValueTuple>>(),

        _ => vec![],
    }
}

/// Check if a given key/value pair matches some lenses.
fn match_lenses(lenses: &[KeyValueTuple], (key, value): (&String, &Value)) -> bool {
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

pub fn apply_filter(filters: &[Filter], json: &Value) -> ExtendedSelections {
    // Apply the filter iff the provided JSON value is an array.
    match json.as_array() {
        Some(array) => {
            let selections: Vec<Selections> = array
                .par_iter()
                .map(|partial_json| {
                    filters.iter().try_fold(
                        vec![partial_json.clone()],
                        |acc: Vec<Value>, filter| {
                            match filter {
                                Filter::Default(selector) => {
                                    if let Some(value) = acc.last() {
                                        get_selection(&[selector.to_owned()], value)
                                    } else {
                                        Ok(acc)
                                    }
                                }
                                Filter::Lens(selector) => {
                                    if !filters.is_empty() && partial_json.is_object() {
                                        // We can safely unwrap here based on the conditional
                                        // above.
                                        let object = partial_json.as_object().unwrap();

                                        let lenses = get_lenses(selector);

                                        Ok(
                                            if object
                                                .iter()
                                                .any(|key_value| match_lenses(&lenses, key_value))
                                            {
                                                acc
                                            } else {
                                                vec![]
                                            },
                                        )
                                    } else {
                                        Ok(acc)
                                    }
                                }
                            }
                        },
                    )
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
            if filters.is_empty() {
                Ok(MaybeArray::NonArray(vec![json.clone()]))
            } else {
                Err(String::from("A filter can only be applied to an array"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_apply_filter() {
        assert_eq!(
            Ok(MaybeArray::Array(vec![
                json!("A"),
                json!("B"),
                json!("C"),
                json!("D"),
                json!("E")
            ])),
            apply_filter(&[], &json!(["A", "B", "C", "D", "E"]),)
        );

        assert_eq!(
            Err(String::from("Root element is not an array")),
            apply_filter(
                &[Filter::Default(Selector::Range((Some(1), Some(3))))],
                &json!(["A", "B", "C", "D", "E"]),
            )
        );

        assert_eq!(
            Ok(MaybeArray::Array(vec![json!(["B", "C", "D"])])),
            apply_filter(
                &[Filter::Default(Selector::Range((Some(1), Some(3))))],
                &json!([["A", "B", "C", "D", "E"]]),
            )
        );

        assert_eq!(
            Ok(MaybeArray::Array(vec![
                json!({ "A": 10, "B": 20, "C": 30, "D": 40, "E": 50 })
            ])),
            apply_filter(
                &[Filter::Lens(Selector::Object(vec![InnerObject::KeyValue(
                    "A".to_string(),
                    Some("10".to_string()),
                )]))],
                &json!([{ "A": 10, "B": 20, "C": 30, "D": 40, "E": 50 }]),
            )
        );

        assert_eq!(
            Ok(MaybeArray::Array(vec![])),
            apply_filter(
                &[Filter::Lens(Selector::Object(vec![InnerObject::KeyValue(
                    "A".to_string(),
                    Some("11".to_string()),
                )]))],
                &json!([{ "A": 10, "B": 20, "C": 30, "D": 40, "E": 50 }]),
            )
        );
    }

    #[test]
    fn not_array_apply_filter() {
        assert_eq!(
            Ok(MaybeArray::NonArray(vec![json!("foo")])),
            apply_filter(&[], &json!("foo"),)
        );
        assert_eq!(
            Err(String::from("A filter can only be applied to an array")),
            apply_filter(
                &[Filter::Default(Selector::Default("foo".to_string()))],
                &json!("foo"),
            )
        );
    }
}
