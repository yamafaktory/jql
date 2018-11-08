use serde_json::Value;
use types::Selection;

pub fn flatten_group(selection: Selection) -> Selection {
    match selection {
        Ok(json) => Ok(json
            .into_iter()
            .map(|value| {
                if value.is_array() {
                    Value::Null
                } else {
                    value
                }
            }).collect::<Vec<Value>>()),
        Err(error) => Err(error),
    }
}
