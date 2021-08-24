use rayon::prelude::*;
use serde_json::json;
use serde_json::Value;

/// Flatten nested arrays provided as a JSON value.
pub fn flatten_json_array(value: &Value) -> Value {
    json!(value
        .as_array()
        .unwrap()
        .par_iter()
        .fold_with(Vec::new(), |mut acc: Vec<Value>, inner_value: &Value| {
            if inner_value.is_array() {
                let recursive_flatten = flatten_json_array(inner_value);
                if recursive_flatten.is_array() {
                    acc.append(&mut recursive_flatten.as_array().unwrap().clone());
                } else {
                    acc.push(inner_value.clone());
                }
            } else {
                acc.push(inner_value.clone());
            }
            acc
        })
        .flatten()
        .collect::<Vec<Value>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"[[[[[[[[[[[[[[1]]]]]]]]]]]]], [[[[[2]]]], 3], null]"#;

    #[test]
    fn get_flatten_value() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        assert_eq!(json!([1, 2, 3, null]), flatten_json_array(&json));
    }
}
