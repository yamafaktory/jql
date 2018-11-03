use serde_json::Value;
use types::Selectors;
use utils::display_node_or_range;

/// Walks through a JSON array.
pub fn array_walker(
    map_index: usize,
    array_index: isize,
    inner_json: &Value,
    raw_selector: &str,
    selectors: &Selectors,
) -> Result<Value, String> {
    // A Negative index has been provided.
    if (array_index).is_negative() {
        return Err(String::from("Invalid negative array index"));
    }

    // A JSON null value has been found (array).
    if inner_json[array_index as usize] == Value::Null {
        let error_message = match inner_json.as_array() {
            // Trying to access an out of bound index on a node
            // or on the root element.
            Some(array) => {
                if selectors.len() == 1 {
                    [
                        "Index (",
                        raw_selector,
                        ") is out of bound, root element has a length of",
                        &(array.len()).to_string(),
                    ]
                        .join(" ")
                } else {
                    [
                        "Index (",
                        raw_selector,
                        ") is out of bound,",
                        &display_node_or_range(
                            &selectors[map_index - 1],
                            false,
                        ),
                        "has a length of",
                        &(array.len()).to_string(),
                    ]
                        .join(" ")
                }
            }
            // Trying to access an index on a node which is not
            // an array.
            None => {
                if selectors.len() == 1 || map_index == 0 {
                    ["Root element is not an array"].join(" ")
                } else {
                    [
                        &display_node_or_range(&selectors[map_index - 1], true),
                        "is not an array",
                    ]
                        .join(" ")
                }
            }
        };

        return Err(error_message);
    }

    // Match found.
    Ok(inner_json[array_index as usize].clone())
}
