use std::num::NonZeroUsize;

use jql_parser::tokens::{Index, Range};
use serde_json::{json, Value};

use crate::errors::JqlRunnerError;

/// Takes an `Index` and a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_index(index: &Index, json: &Value) -> Result<Value, JqlRunnerError> {
    let num: usize = index.clone().into();

    if let Some(value) = json.get(num) {
        Ok(value.clone())
    } else {
        Err(JqlRunnerError::IndexNotFoundError {
            index: num,
            parent: json.to_string(),
        })
    }
}

/// Takes a vector of `Index` and a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_indexes(indexes: &[Index], json: &Value) -> Result<Value, JqlRunnerError> {
    let values: Vec<Value> = indexes
        .iter()
        .try_fold(vec![], |mut acc: Vec<Value>, index| {
            acc.push(get_index(index, json)?);

            Ok::<Vec<Value>, JqlRunnerError>(acc)
        })?;

    Ok(json!(values))
}

/// Takes a key and a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_key(key: &str, json: &Value) -> Result<Value, JqlRunnerError> {
    if let Some(value) = json.get(key) {
        Ok(value.clone())
    } else {
        Err(JqlRunnerError::KeyNotFoundError {
            key: key.to_string(),
            parent: json.to_string(),
        })
    }
}

/// Takes a `Range` and a JSON `Value`.
/// Returns a JSON `Value` or an error.
pub(crate) fn get_range(range: &Range, json: &mut Value) -> Result<Value, JqlRunnerError> {
    let array = as_array(json)?;

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
            parent: json.to_string(),
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

fn as_array(json: &mut Value) -> Result<&mut Vec<Value>, JqlRunnerError> {
    if let Some(values) = json.as_array_mut() {
        Ok(values)
    } else {
        Err(JqlRunnerError::UnknownError)
    }
}

#[cfg(test)]
mod tests {
    use jql_parser::tokens::{Index, Range};
    use serde_json::json;

    use super::get_key;
    use crate::{errors::JqlRunnerError, json::get_range};

    #[test]
    fn check_get_key() {
        let value = json!({ "a": 1 });

        assert_eq!(get_key("a", &value), Ok(json!(1)));
        assert_eq!(
            get_key("b", &value),
            Err(JqlRunnerError::KeyNotFoundError {
                key: "b".to_string(),
                parent: value.to_string()
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
                parent: value.to_string()
            })
        );
    }
}
