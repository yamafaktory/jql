use crate::types::{Display, InnerObject};

/// Convert an array selector to a readable string.
pub fn display_array_selector(capitalized: bool) -> String {
    String::from(if capitalized { "Array" } else { "array" })
}

/// Convert a default selector to a readable string.
pub fn display_default_selector(value: &str, capitalized: bool) -> String {
    [
        if capitalized {
            r#"Node ""#
        } else {
            r#"node ""#
        },
        value,
        r#"""#,
    ]
    .join("")
}

/// Convert an index selector to a readable string.
pub fn display_index_selector(indexes: &[usize], capitalized: bool) -> String {
    if indexes.len() == 1 {
        [
            if capitalized { "Index [" } else { "index [" },
            indexes[0].to_string().as_str(),
            "]",
        ]
        .join("")
    } else {
        [
            if capitalized {
                "Indexes ["
            } else {
                "indexes ["
            },
            indexes
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(",")
                .as_str(),
            "]",
        ]
        .join("")
    }
}

/// Convert an object selector to a readable string.
pub fn display_object_selector(properties: &[InnerObject], capitalized: bool) -> String {
    if properties.len() == 1 {
        [
            if capitalized {
                "Property {"
            } else {
                "property {"
            },
            properties[0].as_str(false).as_str(),
            "}",
        ]
        .join("")
    } else {
        [
            if capitalized {
                "Properties {"
            } else {
                "properties {"
            },
            // properties.join(",").as_str(false),
            "}",
        ]
        .join("")
    }
}

/// Convert a range selector to a readable string.
pub fn display_range_selector(
    (start, end): (Option<usize>, Option<usize>),
    capitalized: bool,
) -> String {
    let position_to_string = |position: Option<usize>| match position {
        Some(value) => value.to_string(),
        None => String::from(""),
    };
    let (start, end) = (position_to_string(start), position_to_string(end));

    [
        if capitalized { "Range [" } else { "range [" },
        start.as_str(),
        ":",
        end.as_str(),
        "]",
    ]
    .join("")
}
