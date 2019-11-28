use crate::apply_filter::apply_filter;
use crate::flatten_json_array::flatten_json_array;
use crate::get_selection::get_selection;
use crate::types::{Group, MaybeArray, Selection};
use serde_json::json;
use serde_json::Value;

/// Walks through a group.
pub fn group_walker(
    (spread, root, selectors, filters): &Group,
    json: &Value,
) -> Selection {
    // Empty group, return early.
    if selectors.is_empty() && root.is_none() {
        return Err(String::from("Empty group"));
    }

    match get_selection(&selectors, &json) {
        Ok(ref items) => {
            // Check for an empty selection, in this case we assume that the
            // user expects to get back the complete raw JSON for this group.
            let output_json = if items.is_empty() {
                json.clone()
            } else {
                json!(items.last())
            };

            let is_spreading = spread.is_some();

            match apply_filter(&filters, &output_json) {
                Ok(filtered) => match filtered {
                    MaybeArray::Array(array) => Ok(if is_spreading {
                        flatten_json_array(&json!(array))
                    } else {
                        json!(array)
                    }),
                    MaybeArray::NonArray(single_value) => {
                        if is_spreading {
                            Err(String::from("Only arrays can be flattened"))
                        } else {
                            // We know that we are holding a single value
                            // wrapped inside a MaybeArray::NonArray enum.
                            // We need to pick the first item of the vector.
                            Ok(json!(single_value[0]))
                        }
                    }
                },
                Err(error) => Err(error),
            }
        }
        Err(items) => Err(items),
    }
}
