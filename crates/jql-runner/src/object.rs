use std::{
    num::NonZeroUsize,
    string::ToString,
};

use indexmap::{
    IndexMap,
    IndexSet,
};
use jql_parser::tokens::{
    Index,
    Range,
};
use rayon::prelude::*;
use serde_json::{
    json,
    Map,
    Value,
};

use crate::errors::JqlRunnerError;

/// Takes a reference of a JSON `Value` and returns a reference of a JSON `Map`
/// or an error.
fn as_object_mut(json: &mut Value) -> Result<&mut Map<String, Value>, JqlRunnerError> {
    if json.is_object() {
        // We can safely unwrap here since this is an object.
        Ok(json.as_object_mut().unwrap())
    } else {
        Err(JqlRunnerError::InvalidObjectError(json.clone()))
    }
}

/// Takes a key as a string slice and a reference of a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_object_key(key: &str, json: &Value) -> Result<Value, JqlRunnerError> {
    if !json.is_object() {
        return Err(JqlRunnerError::InvalidObjectError(json.clone()));
    }

    json.get(key)
        .ok_or_else(|| JqlRunnerError::KeyNotFoundError {
            key: key.to_string(),
            parent: json.clone(),
        })
        .cloned()
}

/// Takes a key as a string slice and a reference of a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_object_multi_key(
    keys: &[&str],
    json: &mut Value,
) -> Result<Value, JqlRunnerError> {
    let len = keys.len();

    let (mut result, found_keys) = as_object_mut(json)?
        .iter_mut()
        .par_bridge()
        .try_fold_with(
            (IndexMap::with_capacity(len), IndexSet::with_capacity(len)),
            |mut acc: (IndexMap<usize, Value>, IndexSet<String>), (key, value)| {
                if let Some(index) = keys.iter().position(|s| s == key) {
                    acc.0.insert(index, value.clone());
                    acc.1.insert(key.to_string());
                }

                Ok::<(IndexMap<usize, Value>, IndexSet<String>), JqlRunnerError>(acc)
            },
        )
        .try_reduce(
            || (IndexMap::with_capacity(len), IndexSet::with_capacity(len)),
            |mut a, b| {
                a.0.extend(b.0);
                a.1.extend(b.1);

                Ok(a)
            },
        )?;

    let keys_set: IndexSet<String> = keys.iter().map(ToString::to_string).collect();
    let mut keys_not_found: Vec<String> = found_keys
        .symmetric_difference(&keys_set)
        .map(ToString::to_string)
        .collect();

    if !keys_not_found.is_empty() {
        keys_not_found.sort();
        return Err(JqlRunnerError::MultiKeyNotFoundError {
            keys: keys_not_found,
            parent: json.clone(),
        });
    }

    // Restore the original order.
    result.par_sort_keys();

    let new_map = result
        .into_iter()
        .fold(Map::with_capacity(len), |mut acc, (index, value)| {
            acc.insert(keys[index].to_owned(), value);

            acc
        });

    Ok(json!(new_map))
}

/// Takes a mutable reference of a JSON `Value`.
/// Returns a flattened object as a JSON `Value`.
pub(crate) fn get_flattened_object(json: &Value) -> Value {
    let mut flattened = Map::<String, Value>::new();

    flatten_value(json, String::new(), 0, &mut flattened);

    json!(flattened)
}

/// Internal utility for `flatten_json_object`.
fn flatten_value(
    json: &Value,
    parent_key: String,
    depth: usize,
    flattened: &mut Map<String, Value>,
) {
    if let Some(value) = json.as_object() {
        flatten_object(value, &parent_key, depth, flattened);
    } else {
        flattened.insert(parent_key, json.clone());
    }
}

/// Internal utility for `flatten_json_object`.
fn flatten_object(
    map: &Map<String, Value>,
    parent_key: &str,
    depth: usize,
    flattened: &mut Map<String, Value>,
) {
    for (k, v) in map {
        let parent_key = if depth > 0 {
            format!("{}{}{}", parent_key, ".", k)
        } else {
            k.to_string()
        };

        flatten_value(v, parent_key, depth + 1, flattened);
    }
}

/// Takes a slice of `Index` and a mutable reference of a JSON `Value`.
/// Returns a reference of a JSON `Value` or an error.
pub(crate) fn get_object_indexes(
    indexes: &[Index],
    json: &mut Value,
) -> Result<Value, JqlRunnerError> {
    let mut_object = as_object_mut(json)?;

    if mut_object.is_empty() {
        return Ok(json!({}));
    }

    let len = indexes.len();
    // We can safely unwrap since indexes can't be empty.
    let max: usize = (*indexes.iter().max().unwrap()).into();

    if max + 1 > mut_object.len() {
        return Err(JqlRunnerError::IndexOutOfBoundsError {
            index: max,
            parent: json.clone(),
        });
    }

    let mut result = mut_object
        .iter_mut()
        .enumerate()
        .par_bridge()
        .try_fold_with(
            IndexMap::with_capacity(len),
            |mut acc: IndexMap<usize, (String, Value)>, (index, (key, value))| {
                if let Some(index) = indexes.iter().position(|i| {
                    let num: usize = (*i).into();

                    num == index
                }) {
                    acc.insert(index, (key.to_string(), value.clone()));
                }

                Ok::<IndexMap<usize, (String, Value)>, JqlRunnerError>(acc)
            },
        )
        .try_reduce(
            || IndexMap::with_capacity(len),
            |mut a, b| {
                a.extend(b);

                Ok(a)
            },
        )?;

    // Restore the original order.
    result.par_sort_keys();

    let new_map = result
        .into_iter()
        .fold(Map::with_capacity(len), |mut acc, (_, (key, value))| {
            acc.insert(key, value);

            acc
        });

    Ok(json!(new_map))
}

/// Takes a reference of a `Range` and a mutable reference of a JSON `Value`.
/// Returns a reference of a JSON `Value` or an error.
pub(crate) fn get_object_range(range: &Range, json: &mut Value) -> Result<Value, JqlRunnerError> {
    let mut_object = as_object_mut(json)?;

    if mut_object.is_empty() {
        return Ok(json!({}));
    }

    let len = mut_object.len();
    // Object's length can't be zero so we can safely unwrap here.
    let non_zero_len = NonZeroUsize::new(len).unwrap();
    let (start, end) = range.to_boundaries(non_zero_len);

    // Out of bounds.
    if start + 1 > len || end + 1 > len {
        return Err(JqlRunnerError::RangeOutOfBoundsError {
            start,
            end,
            parent: json.clone(),
        });
    }

    let is_natural_order = start < end;

    let mut result = mut_object
        .iter_mut()
        .enumerate()
        .par_bridge()
        .try_fold_with(
            IndexMap::with_capacity(len),
            |mut acc: IndexMap<usize, (String, Value)>, (index, (key, value))| {
                if (is_natural_order && index >= start && index <= end)
                    || (!is_natural_order && index >= end && index <= start)
                {
                    acc.insert(index, (key.to_string(), value.clone()));
                }

                Ok::<IndexMap<usize, (String, Value)>, JqlRunnerError>(acc)
            },
        )
        .try_reduce(
            || IndexMap::with_capacity(len),
            |mut a, b| {
                a.extend(b);

                Ok(a)
            },
        )?;

    // Restore the original order.
    result.par_sort_keys();

    // Reverse if not in natural order.
    if !is_natural_order {
        result.reverse();
    }

    let new_map = result
        .into_iter()
        .fold(Map::with_capacity(len), |mut acc, (_, (key, value))| {
            acc.insert(key, value);

            acc
        });

    Ok(json!(new_map))
}

#[cfg(test)]
mod tests {
    use jql_parser::tokens::{
        Index,
        Range,
    };
    use serde_json::{
        json,
        Value,
    };

    use super::{
        get_flattened_object,
        get_object_indexes,
        get_object_key,
        get_object_multi_key,
        get_object_range,
    };
    use crate::errors::JqlRunnerError;

    /// If we perform a direct comparison between the processed value and
    /// the expected value from the `json!` macro, we might get a false
    /// positive since the order of the keys is not checked on equality.
    fn assert_string_eq(processed: Result<Value, JqlRunnerError>, expected: Value) {
        let processed = processed.unwrap();
        let processed = serde_json::to_string(&processed).unwrap();
        let expected = serde_json::to_string(&expected).unwrap();

        assert_eq!(processed, expected);
    }

    #[test]
    fn check_get_object_key() {
        let value = json!({ "a": 1 });

        assert_eq!(get_object_key("a", &value), Ok(json!(1)));
        assert_eq!(
            get_object_key("b", &value),
            Err(JqlRunnerError::KeyNotFoundError {
                key: "b".to_string(),
                parent: value
            })
        );
    }

    #[test]
    fn check_get_object_multi_key() {
        let value = json!({ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 });
        assert_eq!(
            get_object_multi_key(&["a", "b", "c"], &mut value.clone()),
            Ok(json!({ "a": 1, "b": 2, "c": 3 }))
        );
        assert_string_eq(
            get_object_multi_key(&["c", "a", "b"], &mut value.clone()),
            json!({"c": 3, "a": 1, "b": 2}),
        );
        assert_eq!(
            get_object_multi_key(&["w", "a", "t"], &mut value.clone()),
            Err(JqlRunnerError::MultiKeyNotFoundError {
                keys: vec!["t".to_string(), "w".to_string()],
                parent: value,
            })
        );

        let value = json!(1);
        assert_eq!(
            get_object_multi_key(&["a", "b", "c"], &mut value.clone()),
            Err(JqlRunnerError::InvalidObjectError(value))
        );
    }

    #[test]
    fn check_get_flattened_object() {
        assert_eq!(
            get_flattened_object(
                &json!({ "a": { "c": false }, "b": { "d": { "e": { "f": 1, "g": { "h": 2 }} } } })
            ),
            json!({
              "a.c": false,
              "b.d.e.f": 1,
              "b.d.e.g.h": 2
            })
        );
    }

    #[test]
    fn check_get_object_indexes() {
        let value = json!({ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 });

        assert_string_eq(
            get_object_indexes(
                &[Index::new(4), Index::new(2), Index::new(0)],
                &mut value.clone(),
            ),
            json!({ "e": 5, "c": 3, "a": 1 }),
        );
        assert_eq!(
            get_object_indexes(
                &[Index::new(4), Index::new(2), Index::new(10)],
                &mut value.clone()
            ),
            Err(JqlRunnerError::IndexOutOfBoundsError {
                index: 10,
                parent: value,
            })
        );
    }

    #[test]
    fn check_get_object_range() {
        let value = json!({ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 });

        assert_eq!(
            get_object_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(2))),
                &mut json!({})
            ),
            Ok(json!({}))
        );

        assert_string_eq(
            get_object_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(2))),
                &mut value.clone(),
            ),
            json!({ "a": 1, "b": 2, "c": 3 }),
        );
        assert_string_eq(
            get_object_range(
                &Range::new(Some(Index::new(2)), Some(Index::new(0))),
                &mut value.clone(),
            ),
            json!({ "c": 3, "b": 2, "a": 1 }),
        );
        assert_eq!(
            get_object_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(0))),
                &mut value.clone()
            ),
            Ok(json!({ "a": 1 }))
        );
        assert_string_eq(
            get_object_range(&Range::new(None, Some(Index::new(4))), &mut value.clone()),
            json!({ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 }),
        );
        assert_eq!(
            get_object_range(&Range::new(Some(Index::new(4)), None), &mut value.clone()),
            Ok(json!({ "e": 5 }))
        );
        assert_eq!(
            get_object_range(&Range::new(None, Some(Index::new(5))), &mut value.clone()),
            Err(JqlRunnerError::RangeOutOfBoundsError {
                start: 0,
                end: 5,
                parent: value
            })
        );

        let value = json!(1);
        assert_eq!(
            get_object_range(&Range::new(None, Some(Index::new(5))), &mut value.clone()),
            Err(JqlRunnerError::InvalidObjectError(value))
        );
    }
}
