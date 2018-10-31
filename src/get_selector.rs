use lazy_static::lazy_static;
use regex::Regex;
use types::Selector;

/// Get the trimmed text of the match with a default of an empty
/// string if the group didn't participate in the match.
pub fn get_selector(capture: &str) -> Selector {
    let capture = capture.trim();

    if capture.starts_with('\"') {
        // let cap_string = String::from(cap);
        // Drop the enclosing double quotes in this case.
        // let inner_cap = cap_string[1..cap_string.len() - 1];
        Selector::Default(String::from(&capture[1..capture.len() - 1]))
    } else {
        // Array range, e.g. 0:3.
        lazy_static! {
            static ref RANGE_REGEX: Regex = Regex::new(r"(\d+):(\d+)").unwrap();
        }

        let ranges: Vec<(&str, &str)> = RANGE_REGEX
            .captures_iter(capture)
            .map(|capture| {
                (
                    capture.get(1).map_or("", |m| m.as_str()),
                    capture.get(2).map_or("", |m| m.as_str()),
                )
            }).collect();
        if ranges.is_empty() {
            // Returns the initial captured value.
            Selector::Default(String::from(capture))
        } else {
            // Returns the range as a tuple of the form (start,end).
            let (start, end) = &ranges[0];
            Selector::Range((
                usize::from_str_radix(start, 10).unwrap(),
                usize::from_str_radix(end, 10).unwrap(),
            ))
        }
    }
}
