// Ref: https://github.com/vtselfa/flatten-json-object/blob/master/src/lib.rs
use serde_json::{json, Map, Value};

/// Flatten nested objects provided as a JSON value.
pub fn flatten_json_object(value: &Value) -> Value {
    let mut flat = Map::<String, Value>::new();
    flatten_value(value, "".to_owned(), 0, &mut flat);
    json!(flat)
}

fn flatten_value(
    value: &Value,
    parent_key: String,
    depth: u32,
    flattened: &mut Map<String, Value>,
) {
    if let Some(value) = value.as_object() {
        flatten_object(value, &parent_key, depth, flattened);
    } else {
        flattened.insert(parent_key, value.clone());
    }
}

fn flatten_object(
    value: &Map<String, Value>,
    parent_key: &str,
    depth: u32,
    flattened: &mut Map<String, Value>,
) {
    for (k, v) in value.iter() {
        let parent_key = if depth > 0 {
            format!("{}{}{}", parent_key, ".", k)
        } else {
            k.to_string()
        };
        flatten_value(v, parent_key, depth + 1, flattened);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"{"foo": {"bar": false}, "foobar": {"bar": true}}"#;

    #[test]
    fn get_flatten_value() {
        let json: Value = serde_json::from_str(DATA).unwrap();
        assert_eq!(
            json!({
              "foo.bar": false,
              "foobar.bar": true
            }),
            flatten_json_object(&json)
        );
    }
}
