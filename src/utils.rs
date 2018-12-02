use types::Selector;

/// Convert a range to a readable string.
pub fn display_range_selector(
    (start, end): (usize, usize),
    capitalized: bool,
) -> String {
    [
        if capitalized {
            r#"Range ""#
        } else {
            r#"range ""#
        },
        start.to_string().as_str(),
        ":",
        end.to_string().as_str(),
        r#"""#,
    ]
        .join("")
}

/// Convert a range to a readable string.
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

/// Return the node or the range of Selector as a string.
pub fn display_node_or_range(selector: &Selector, capitalized: bool) -> String {
    match selector {
        Selector::Default(value) => {
            display_default_selector(&value.clone(), capitalized)
        }
        Selector::Range(range) => display_range_selector(*range, capitalized),
    }
}
