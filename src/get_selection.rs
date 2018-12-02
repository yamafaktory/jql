use array_walker::array_walker;
use range_selector::range_selector;
use serde_json::Value;
use types::{Selection, Selector, Selectors};
use utils::display_node_or_range;

/// Returns a selection based on selectors and a JSON content as a Result of
/// values or an Err early on, stopping the iteration as soon as the latter is
/// encountered.
pub fn get_selection(selectors: &Selectors, json: &Value) -> Selection {
    // Local copy of the original JSON that will be reused in the loop.
    let mut inner_json = json.clone();

    selectors
        .iter()
        .enumerate()
        .map(|(map_index, current_selector)| -> Result<Value, String> {
            match current_selector {
                // Default selector.
                Selector::Default(raw_selector) => {
                    // Array case.
                    if let Ok(array_index) = raw_selector.parse::<isize>() {
                        return match array_walker(
                            map_index,
                            array_index,
                            &inner_json.clone(),
                            raw_selector,
                            &selectors,
                        ) {
                            Ok(json) => {
                                inner_json = json.clone();
                                Ok(json.clone())
                            }
                            Err(error) => Err(error),
                        };
                    }

                    // No JSON value has been found (non array).
                    if inner_json.get(raw_selector).is_none() {
                        if map_index == 0 {
                            Err([
                                r#"Node ""#,
                                raw_selector,
                                r#"" not found on the parent element"#,
                            ]
                                .join(""))
                        } else {
                            Err([
                                r#"Node ""#,
                                raw_selector,
                                r#"" not found on parent "#,
                                &display_node_or_range(
                                    &selectors[map_index - 1],
                                    false,
                                ),
                            ]
                                .join(""))
                        }
                    // Default case.
                    } else {
                        inner_json = inner_json[raw_selector].clone();
                        Ok(inner_json.clone())
                    }
                }

                // Range selector.
                Selector::Range((start, end)) => match range_selector(
                    map_index,
                    &inner_json.clone(),
                    *start,
                    *end,
                    &selectors,
                    if map_index == 0 {
                        None
                    } else {
                        Some(&selectors[map_index - 1])
                    },
                ) {
                    Ok(json) => {
                        inner_json = json.clone();
                        Ok(json.clone())
                    }
                    Err(error) => Err(error),
                },
            }
        }).collect()
}
