use std::num::NonZeroUsize;

use jql_parser::tokens::{
    Index,
    Range,
};
use rayon::prelude::*;
use serde_json::{
    json,
    Value,
};

use crate::errors::JqlRunnerError;

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

/// Takes a reference of an `Index` and a JSON `Value`.
/// Returns a reference of a JSON `Value` or an error.
fn get_array_index(index: &Index, json: &Value) -> Result<Value, JqlRunnerError> {
    let num: usize = index.clone().into();

    if let Some(value) = json.get(num) {
        Ok(value.clone())
    } else {
        Err(JqlRunnerError::IndexOutOfBoundsError {
            index: num,
            parent: json.clone(),
        })
    }
}

/// Takes a slice of `Index` and a reference of a JSON `Value`.
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

/// Takes a reference of a `Range` and a mutable reference of a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_array_range(range: &Range, json: &mut Value) -> Result<Value, JqlRunnerError> {
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
/// Note: the runner checks that the input is a JSON array.
pub(crate) fn get_flattened_array(json: &Value) -> Result<Value, JqlRunnerError> {
    let result = json
        .as_array()
        .unwrap()
        .par_iter()
        .try_fold_with(vec![], |mut acc: Vec<Value>, inner_value| {
            if inner_value.is_array() {
                let mut flattened = get_flattened_array(inner_value)?;
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
    use jql_parser::tokens::{
        Index,
        Range,
    };
    use serde_json::json;

    use super::{
        get_array_index,
        get_array_indexes,
        get_array_range,
        get_flattened_array,
    };
    use crate::errors::JqlRunnerError;

    #[test]
    fn check_get_array_index() {
        let value = json!(["a", "b", "c"]);

        assert_eq!(get_array_index(&Index::new(0), &value), Ok(json!("a")));
        assert_eq!(
            get_array_index(&Index::new(3), &value),
            Err(JqlRunnerError::IndexOutOfBoundsError {
                index: 3,
                parent: value
            })
        );
    }

    #[test]
    fn check_get_array_indexes() {
        let value = json!(["a", "b", "c"]);

        assert_eq!(
            get_array_indexes(&[Index::new(0), Index::new(2)], &value),
            Ok(json!(["a", "c"]))
        );
        assert_eq!(
            get_array_indexes(&[Index::new(0), Index::new(3)], &value),
            Err(JqlRunnerError::IndexOutOfBoundsError {
                index: 3,
                parent: value
            })
        );
    }

    #[test]
    fn check_get_array_range() {
        let value = json!(["a", "b", "c", "d", "e"]);

        assert_eq!(
            get_array_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(2))),
                &mut value.clone()
            ),
            Ok(json!(["a", "b", "c"]))
        );
        assert_eq!(
            get_array_range(
                &Range::new(Some(Index::new(2)), Some(Index::new(0))),
                &mut value.clone()
            ),
            Ok(json!(["c", "b", "a"]))
        );
        assert_eq!(
            get_array_range(
                &Range::new(Some(Index::new(0)), Some(Index::new(0))),
                &mut value.clone()
            ),
            Ok(json!(["a"]))
        );
        assert_eq!(
            get_array_range(&Range::new(None, Some(Index::new(4))), &mut value.clone()),
            Ok(json!(["a", "b", "c", "d", "e"]))
        );
        assert_eq!(
            get_array_range(&Range::new(Some(Index::new(4)), None), &mut value.clone()),
            Ok(json!(["e"]))
        );
        assert_eq!(
            get_array_range(&Range::new(None, Some(Index::new(5))), &mut value.clone()),
            Err(JqlRunnerError::RangeOutOfBoundsError {
                start: 0,
                end: 5,
                parent: value
            })
        );

        let value = json!(1);
        assert_eq!(
            get_array_range(&Range::new(None, Some(Index::new(5))), &mut value.clone()),
            Err(JqlRunnerError::InvalidArrayError(value))
        );
    }

    #[test]
    fn check_get_flattened_array() {
        assert_eq!(
            get_flattened_array(&json!([[[[[[[[[[[[[[1]]]]]]]]]]]]], [[[[[2]]]], 3], null])),
            Ok(json!([1, 2, 3, null]))
        );
        assert_eq!(
            get_flattened_array(
                &json!([[[[[[[[[[[[[[{ "a": 1 }]]]]]]]]]]]]], [[[[[{ "b": 2 }]]]], { "c": 3 }], null])
            ),
            Ok(json!([{ "a": 1 }, { "b": 2 }, { "c" : 3 }, null]))
        );
    }
}
