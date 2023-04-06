use jql_parser::tokens::Index;
use serde_json::{
    json,
    Value,
};

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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::get_key;
    use crate::errors::JqlRunnerError;

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
}
