use serde_json::{json, Value};

use crate::{
    apply_filter::apply_filter,
    flatten_json_array::flatten_json_array,
    get_selection::get_selection,
    truncate::truncate_json,
    types::{Group, MaybeArray, Selection},
};

/// Walks through a group.
pub fn group_walker(
    Group {
        filters,
        root,
        selectors,
        spread,
        truncate,
    }: &Group,
    json: &Value,
) -> Selection {
    // Empty group, return early.
    if selectors.is_empty() && root.is_none() {
        return Err(String::from("Empty group"));
    }

    match get_selection(selectors, json) {
        Ok(ref items) => {
            // Check for an empty selection, in this case we assume that the
            // user expects to get back the complete raw JSON for this group.
            let output_json = if items.is_empty() {
                json.clone()
            } else {
                json!(items.last())
            };

            let is_spreading = spread.is_some();

            let output = match apply_filter(filters, &output_json) {
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
            };

            match truncate {
                Some(_) => match output {
                    Ok(value) => Ok(truncate_json(value)),
                    Err(error) => Err(error),
                },
                None => output,
            }
        }
        Err(items) => Err(items),
    }
}
