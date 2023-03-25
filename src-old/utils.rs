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
            properties
                .iter()
                .map(|id| id.as_str(false) + ",")
                .collect::<String>()
                .as_str(),
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
        None => String::new(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_selector() {
        assert_eq!("Array", display_array_selector(true));
        assert_eq!("array", display_array_selector(false));
    }

    #[test]
    fn default_selector() {
        assert_eq!("Node \"foo\"", display_default_selector("foo", true));
        assert_eq!("node \"foo\"", display_default_selector("foo", false));
    }

    #[test]
    fn index_selector() {
        assert_eq!("Index [1]", display_index_selector(&[1], true));
        assert_eq!("index [1]", display_index_selector(&[1], false));
        assert_eq!("Indexes [1,2,3]", display_index_selector(&[1, 2, 3], true));
        assert_eq!("indexes [1,2,3]", display_index_selector(&[1, 2, 3], false));
    }

    #[test]
    fn object_selector() {
        assert_eq!(
            "Property {foo}",
            display_object_selector(&[InnerObject::KeyValue("foo".to_string(), None)], true)
        );
        assert_eq!(
            "property {foo}",
            display_object_selector(&[InnerObject::KeyValue("foo".to_string(), None)], false)
        );
        assert_eq!(
            "Properties {array,foo,index [1],range [1:2],}",
            display_object_selector(
                &[
                    InnerObject::Array,
                    InnerObject::KeyValue("foo".to_string(), None),
                    InnerObject::Index([1].to_vec()),
                    InnerObject::Range((Some(1), Some(2)))
                ],
                true
            )
        );
        assert_eq!(
            "properties {array,foo,index [1],range [1:2],}",
            display_object_selector(
                &[
                    InnerObject::Array,
                    InnerObject::KeyValue("foo".to_string(), None),
                    InnerObject::Index([1].to_vec()),
                    InnerObject::Range((Some(1), Some(2)))
                ],
                false
            )
        );
    }

    #[test]
    fn range_selector() {
        assert_eq!(
            "Range [1:2]",
            display_range_selector((Some(1), Some(2)), true)
        );
        assert_eq!(
            "range [1:2]",
            display_range_selector((Some(1), Some(2)), false)
        );
        assert_eq!("Range [1:]", display_range_selector((Some(1), None), true));
        assert_eq!("Range [:2]", display_range_selector((None, Some(2)), true));
        assert_eq!("Range [:]", display_range_selector((None, None), true));
    }
}
