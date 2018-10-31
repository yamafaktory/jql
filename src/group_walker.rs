use apply_filter::apply_filter;
use get_selection::get_selection;
use get_selector::get_selector;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use serde_json::Value;
use types::{Selection, Selector};
/// Walks through a group.
pub fn group_walker(
    capture: &regex::Captures<'_>,
    filter: Option<&str>,
    json: &Value,
) -> Selection {
    lazy_static! {
        static ref SUB_GROUP_REGEX: Regex =
            Regex::new(r#"("[^"]+")|([^.]+)"#).unwrap();
    }

    let group = capture.get(0).map_or("", |m| m.as_str()).trim();

    // Empty group, return early.
    if group.is_empty() {
        return Err(String::from("Empty group"));
    }

    // Capture sub-groups of double quoted selectors and simple ones surrounded
    // by dots on the group itself.
    let selectors: Vec<Selector> = SUB_GROUP_REGEX
        .captures_iter(group)
        .map(|capture| get_selector(capture.get(0).map_or("", |m| m.as_str())))
        .collect();

    // Perform the same operation on the filter.
    let filter_selectors = match filter {
        Some(filter) => Some(
            SUB_GROUP_REGEX
                .captures_iter(filter)
                .map(|capture| {
                    get_selector(capture.get(0).map_or("", |m| m.as_str()))
                }).collect::<Vec<Selector>>(),
        ),
        None => None,
    };

    // Returns a Result of values or an Err early on, stopping the iteration
    // as soon as the latter is encountered.
    let items: Selection = get_selection(&selectors, &json);

    // Check for empty selection, in this case we assume that the user expects
    // to get back the complete raw JSON back for this group.
    match items {
        Ok(items) => {
            if items.is_empty() {
                apply_filter(&json.clone(), &filter_selectors)
            } else {
                apply_filter(&json!(items.last()), &filter_selectors)
            }
        }
        Err(items) => Err(items),
    }
}
