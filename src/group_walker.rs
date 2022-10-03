use serde_json::{json, Value};

use crate::{
    apply_filter::apply_filter,
    flatten_json_array::flatten_json_array,
    flatten_json_object::flatten_json_object,
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
                        Ok(if is_spreading {
                            flatten_json_object(&json!(single_value[0]))
                        } else {
                            // We know that we are holding a single value
                            // wrapped inside a MaybeArray::NonArray enum.
                            // We need to pick the first item of the vector.
                            json!(single_value[0])
                        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Filter, InnerObject, Selector};

    #[test]
    fn empty_group_walker() {
        assert_eq!(
            group_walker(&Default::default(), &json!({ "A": 10, "B": 20, "C": 30 }),),
            Err(String::from("Empty group"))
        );
    }

    #[test]
    fn valid_group_walker() {
        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Index(vec![0, 2])],
                    ..Default::default()
                },
                &json!([["A"], ["B", "C"], ["D", "E"]]),
            ),
            Ok(json!([["A"], ["D", "E"]])),
        );

        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Index(vec![0, 2])],
                    spread: Some(()),
                    ..Default::default()
                },
                &json!([["A"], ["B", "C"], ["D", "E"]]),
            ),
            Ok(json!(["A", "D", "E"])),
        );

        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Index(vec![0, 2])],
                    filters: vec![Filter::Default(Selector::Index(vec![0]))],
                    ..Default::default()
                },
                &json!([["A"], ["B", "C"], ["D", "E"]]),
            ),
            Ok(json!(["A", "D"])),
        );

        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Object(vec![InnerObject::Index(vec![0, 2])])],
                    ..Default::default()
                },
                &json!({ "A": 10, "B": 20, "C": 30 }),
            ),
            Ok(json!({ "0" : 10, "2": 30 })),
        );

        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Index(vec![0, 2])],
                    truncate: Some(()),
                    ..Default::default()
                },
                &json!([["A"], ["B", "C"], ["D", "E"]]),
            ),
            Ok(json!([[], []])),
        );
    }

    #[test]
    fn invalid_group_walker() {
        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Object(vec![InnerObject::Index(vec![10])])],
                    ..Default::default()
                },
                &json!({ "A": 10, "B": 20, "C": 30 }),
            ),
            Err(String::from(
                "Index [10] is out of bound, object contains 3 properties"
            )),
        );

        assert_eq!(
            group_walker(
                &Group {
                    selectors: vec![Selector::Object(vec![InnerObject::Index(vec![0])])],
                    spread: Some(()),
                    ..Default::default()
                },
                &json!({"0" : { "A": 10, "B": 20, "C": 30 }}),
            ),
            Ok(json!({"0.A": 10, "0.B": 20, "0.C": 30})),
        );
    }
}
