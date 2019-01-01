use crate::types::{Display, Selector, Selectors};
use serde_json::json;
use serde_json::Value;

/// Returns a range selection or an error.
pub fn range_selector(
    map_index: usize,
    inner_json: &Value,
    start: Option<usize>,
    end: Option<usize>,
    selectors: &Selectors,
    previous_selector: Option<&Selector>,
) -> Result<Value, String> {
    match inner_json.as_array() {
        Some(json_array) => {
            let (start, end) = (
                match start {
                    Some(start) => start,
                    None => 0,
                },
                match end {
                    Some(end) => end,
                    None => json_array.len() - 1,
                },
            );
            let is_default = start < end;

            // Check the range validity.
            if json_array.len() < start || json_array.len() < (end + 1) {
                return Err(if selectors.len() == 1 {
                    [
                        "Range [",
                        start.to_string().as_str(),
                        ":",
                        end.to_string().as_str(),
                        "] is out of bound, root element has a length of ",
                        &(json_array.len()).to_string(),
                    ]
                    .join("")
                } else {
                    [
                        "Range [",
                        start.to_string().as_str(),
                        ":",
                        end.to_string().as_str(),
                        "] is out of bound, ",
                        &selectors[map_index - 1].as_str(false),
                        " has a length of ",
                        &(json_array.len()).to_string(),
                    ]
                    .join("")
                });
            }

            Ok(if is_default {
                json!(json_array[start..=end])
            } else {
                // Get the normalized slice selection, i.e. from end to start.
                let normalized_range_selection = json!(json_array[end..=start]);
                // Reverse it.
                let reversed_range_selection: Vec<&Value> =
                    normalized_range_selection
                        .as_array()
                        .unwrap()
                        .iter()
                        .rev()
                        .collect();
                json!(reversed_range_selection)
            })
        }
        None => Err([
            (match previous_selector {
                Some(selector) => selector.as_str(true),
                None => String::from("Root element"),
            })
            .as_str(),
            " is not an array",
        ]
        .join("")),
    }
}
