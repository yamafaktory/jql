use serde_json::{json, Map, Value};

/// Truncate a JSON value.
pub fn truncate_json(mut value: Value) -> Value {
    // Closure that returns the primitive of a given value.
    let to_primitive = |value: &Value| match value {
        _ if value.is_array() => json!([]),
        _ if value.is_object() => json!({}),
        _ => value.to_owned(),
    };

    match value {
        _ if value.is_array() => value
            // .clone() 
            .as_array_mut()
            .unwrap()
            .iter()
            .map(|element| to_primitive(element))
            .collect::<Value>(),
        _ if value.is_object() => {
            Value::Object(value.as_object().unwrap().iter().fold(
                Map::new(),
                |mut acc, property| {
                    acc.insert(
                        property.0.to_string(),
                        to_primitive(property.1),
                    );
                    acc
                },
            ))
        }
        _ => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_array() {
        assert_eq!(
            json!([[], {}, "woot", 7, null]),
            truncate_json(json!([[1,2,3], {"foo":"bar"}, "woot", 7, null]))
        );
    }

    #[test]
    fn truncate_object() {
        assert_eq!(
            json!({"foo":[], "bar": {}, "woot": "what", "number": 7, "nothing": null}),
            truncate_json(
                json!({ "foo": [], "bar": {}, "woot": "what", "number": 7, "nothing": null})
            )
        );
    }

    #[test]
    fn truncate_string() {
        assert_eq!(json!("something"), truncate_json(json!("something")));
    }

    #[test]
    fn truncate_number() {
        assert_eq!(json!(7), truncate_json(json!(7)));
    }

    #[test]
    fn truncate_null() {
        assert_eq!(json!(null), truncate_json(json!(null)));
    }
}
