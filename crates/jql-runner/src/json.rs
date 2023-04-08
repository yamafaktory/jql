use std::{collections::HashSet, num::NonZeroUsize, string::ToString};

use jql_parser::tokens::{Index, Range};
use rayon::prelude::*;
use serde_json::{json, Map, Value};

use crate::errors::JqlRunnerError;

/// Takes a reference of a JSON `Value` and returns a reference of a vector of
/// JSON `Value` or an error.
fn as_array(json: &Value) -> Result<&Vec<Value>, JqlRunnerError> {
    json.as_array()
        .ok_or_else(|| JqlRunnerError::InvalidArrayError(json.clone()))
}

/// Takes a mutable reference of JSON `Value` and returns a reference of a
/// mutable vector of JSON `Value` or an error.
fn as_array_mut(json: &mut Value) -> Result<&mut Vec<Value>, JqlRunnerError> {
    if json.is_array() {
        // We can safely unwrap here since this is an array.
        Ok(json.as_array_mut().unwrap())
    } else {
        Err(JqlRunnerError::InvalidArrayError(json.clone()))
    }
}

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

/// Takes a reference of an `Index` and a JSON `Value`.
/// Returns a reference of a JSON `Value` or an error.
fn get_array_index(index: &Index, json: &Value) -> Result<Value, JqlRunnerError> {
    let num: usize = index.clone().into();

    if let Some(value) = json.get(num) {
        Ok(value.clone())
    } else {
        Err(JqlRunnerError::IndexNotFoundError {
            index: num,
            parent: json.clone(),
        })
    }
}

/// Takes a slice of `Index` and a JSON `Value`.
/// Returns a reference of a JSON `Value` or an error.
pub(crate) fn get_array_indexes(indexes: &[Index], json: &Value) -> Result<Value, JqlRunnerError> {
    let values: Vec<Value> = indexes
        .iter()
        .try_fold(vec![], |mut acc: Vec<Value>, index| {
            acc.push(get_array_index(index, json)?);

            Ok::<Vec<Value>, JqlRunnerError>(acc)
        })?;

    Ok(json!(values))
}

/// Takes a key as a string slice and a reference of a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_key(key: &str, json: &Value) -> Result<Value, JqlRunnerError> {
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
pub(crate) fn get_multi_key(keys: &[&str], json: &mut Value) -> Result<Value, JqlRunnerError> {
    let len = keys.len();

    let result = as_object_mut(json)?
        .iter_mut()
        .par_bridge()
        .try_fold_with(
            (Map::with_capacity(len), HashSet::with_capacity(len)),
            |mut acc: (Map<String, Value>, HashSet<String>), (key, value)| {
                if keys.iter().any(|s| s == key) {
                    acc.0.insert(key.to_string(), value.clone());
                    acc.1.insert(key.to_string());
                }

                Ok::<(Map<String, Value>, HashSet<String>), JqlRunnerError>(acc)
            },
        )
        .try_reduce(
            || (Map::with_capacity(keys.len()), HashSet::new()),
            |mut a, b| {
                a.0.extend(b.0);
                a.1.extend(b.1);

                Ok(a)
            },
        )?;

    let keys_set: HashSet<String> = keys.iter().map(ToString::to_string).collect();
    let not_found: Vec<String> = result
        .1
        .symmetric_difference(&keys_set)
        .map(ToString::to_string)
        .collect();

    if !not_found.is_empty() {
        return Err(JqlRunnerError::MultiKeyNotFoundError {
            keys: not_found,
            parent: json.clone(),
        });
    }

    Ok(json!(result.0))
}

/// Takes a reference of a `Range` and a mutable reference of a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_range(range: &Range, json: &mut Value) -> Result<Value, JqlRunnerError> {
    let array = as_array_mut(json)?;

    if array.is_empty() {
        return Ok(json!([]));
    }

    let len = array.len();
    // Array's length can't be zero so we can safely unwrap here.
    let non_zero_len = NonZeroUsize::new(array.len()).unwrap();
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

    let result = if is_natural_order {
        &mut array[start..=end]
    } else {
        &mut array[end..=start]
    };

    if !is_natural_order {
        result.reverse();
    }

    Ok(json!(result))
}

/// Takes a reference of a JSON `Value`.
/// Returns a flattened array as a JSON `Value` or an error.
pub(crate) fn get_flattened(value: &Value) -> Result<Value, JqlRunnerError> {
    let result = as_array(value)?
        .par_iter()
        .try_fold_with(vec![], |mut acc: Vec<Value>, inner_value| {
            if inner_value.is_array() {
                let mut flattened = get_flattened(inner_value)?;
                let result = as_array_mut(&mut flattened)?;

                acc.append(result);
            } else {
                acc.push(inner_value.clone());
            }

            Ok::<Vec<Value>, JqlRunnerError>(acc)
        })
        .try_reduce(
            || vec![],
            |mut a, b| {
                a.extend(b);

                Ok(a)
            },
        )?;

    Ok(json!(result))
}

#[cfg(test)]
mod tests {
    use jql_parser::tokens::{Index, Range};
    use serde_json::json;

    use super::{
        get_array_index, get_array_indexes, get_flattened, get_key, get_multi_key, get_range,
    };
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_get_index() {
        let value = json!(["a", "b", "c"]);

        assert_eq!(get_array_index(&Index::new(0), &value), Ok(json!("a")));
        assert_eq!(
            get_array_index(&Index::new(3), &value),
            Err(JqlRunnerError::IndexNotFoundError {
                index: 3,
                parent: value
            })
        );
    }

    #[test]
    fn check_get_indexes() {
        let value = json!(["a", "b", "c"]);

        assert_eq!(
            get_array_indexes(&[Index::new(0), Index::new(2)], &value),
            Ok(json!(["a", "c"]))
        );
        assert_eq!(
            get_array_indexes(&[Index::new(0), Index::new(3)], &value),
            Err(JqlRunnerError::IndexNotFoundError {
                index: 3,
                parent: value
            })
        );
    }

    #[test]
    fn check_get_key() {
        let value = json!({ "a": 1 });

        assert_eq!(get_key("a", &value), Ok(json!(1)));
        assert_eq!(
            get_key("b", &value),
            Err(JqlRunnerError::KeyNotFoundError {
                key: "b".to_string(),
                parent: value
            })
        );
    }

    #[test]
    fn check_get_range() {
        let value = json!(["a", "b", "c", "d", "e"]);

        assert_eq!(
            get_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(2))),
                &mut value.clone()
            ),
            Ok(json!(["a", "b", "c"]))
        );
        assert_eq!(
            get_range(
                &Range::new(Some(Index::new(2)), Some(Index::new(0))),
                &mut value.clone()
            ),
            Ok(json!(["c", "b", "a"]))
        );
        assert_eq!(
            get_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(0))),
                &mut value.clone()
            ),
            Ok(json!(["a"]))
        );
        assert_eq!(
            get_range(&Range::new(None, Some(Index::new(4))), &mut value.clone()),
            Ok(json!(["a", "b", "c", "d", "e"]))
        );
        assert_eq!(
            get_range(&Range::new(Some(Index::new(4)), None), &mut value.clone()),
            Ok(json!(["e"]))
        );
        assert_eq!(
            get_range(&Range::new(None, Some(Index::new(5))), &mut value.clone()),
            Err(JqlRunnerError::RangeOutOfBoundsError {
                start: 0,
                end: 5,
                parent: value
            })
        );

        let value = json!(1);
        assert_eq!(
            get_range(&Range::new(None, Some(Index::new(5))), &mut value.clone()),
            Err(JqlRunnerError::InvalidArrayError(value))
        );
    }

    #[test]
    fn check_get_flattened() {
        assert_eq!(
            get_flattened(&json!([[[[[[[[[[[[[[1]]]]]]]]]]]]], [[[[[2]]]], 3], null])),
            Ok(json!([1, 2, 3, null]))
        );
        assert_eq!(
            get_flattened(
                &json!([[[[[[[[[[[[[[{ "a": 1 }]]]]]]]]]]]]], [[[[[{ "b": 2 }]]]], { "c": 3 }], null])
            ),
            Ok(json!([{ "a": 1 }, { "b": 2 }, { "c" : 3 }, null]))
        );

        let value = json!(1);
        assert_eq!(
            get_flattened(&value),
            Err(JqlRunnerError::InvalidArrayError(value))
        );
    }

    #[test]
    fn check_get_multi_key() {
        let value = json!({ "a": 1, "b": 2, "c": 3, "d": 4, "e": 5 });

        assert_eq!(
            get_multi_key(&["a", "b", "c"], &mut value.clone()),
            Ok(json!({ "a": 1, "b": 2, "c": 3 }))
        );
        assert_eq!(
            get_multi_key(&["w", "a", "t"], &mut value.clone()),
            Err(JqlRunnerError::MultiKeyNotFoundError {
                keys: vec!["w".to_string(), "t".to_string()],
                parent: value,
            })
        );

        let value = json!(1);
        assert_eq!(
            get_multi_key(&["a", "b", "c"], &mut value.clone()),
            Err(JqlRunnerError::InvalidObjectError(value))
        );
    }
}
