use crate::types::{Display, Selectors};
use serde_json::json;
use serde_json::Value;

/// Walks through a JSON array. Iterate over the indexes of the array, returns
/// a Result of values or an Err early on.
pub fn array_walker(
    map_index: usize,
    array_indexes: &[usize],
    inner_json: &Value,
    selectors: &Selectors,
) -> Result<Value, String> {
    let results: Result<Vec<Value>, String> = array_indexes
        .iter()
        .map(|index| {
            // No JSON value has been found (array).
            if inner_json.get(index).is_none() {
                let error_message = match inner_json.as_array() {
                    // Trying to access an out of bound index on a node
                    // or on the root element.
                    Some(array) => {
                        if selectors.len() == 1 {
                            [
                        "Index [",
                        index.to_string().as_str(),
                        "] is out of bound, root element has a length of ",
                        &(array.len()).to_string(),
                    ]
                    .join("")
                        } else {
                            [
                                "Index [",
                                index.to_string().as_str(),
                                "] is out of bound, ",
                                &selectors[map_index - 1].as_str(false),
                                " has a length of ",
                                &(array.len()).to_string(),
                            ]
                            .join("")
                        }
                    }
                    // Trying to access an index on a node which is not
                    // an array.
                    None => {
                        if selectors.len() == 1 || map_index == 0 {
                            String::from("Root element is not an array")
                        } else {
                            [
                                &selectors[map_index - 1].as_str(true),
                                " is not an array",
                            ]
                            .join("")
                        }
                    }
                };

                return Err(error_message);
            }

            // Match found.
            Ok(inner_json.get(index).unwrap().clone())
        })
        .collect();

    match results {
        Ok(values) => Ok(if array_indexes.len() == 1 {
            // Pick the first one as there is only one index provided.
            values[0].clone()
        } else {
            // Return an array.
            json!(values)
        }),
        Err(error) => Err(error),
    }
}
