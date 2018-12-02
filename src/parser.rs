use lazy_static::lazy_static;
use pest::Parser;
use regex::Regex;
use types::Selector;
use types::{Group, Groups};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroupsParser;

/// Drop the enclosing double quotes of a span and convert it to a default
/// selector.
fn span_to_default(inner_span: &str) -> Selector {
    Selector::Default(
        String::from(&inner_span[1..inner_span.len() - 1])
            .replace(r#"\""#, r#"""#),
    )
}

/// Convert a span to a range selector.
fn span_to_range(inner_span: &str) -> Selector {
    lazy_static! {
        static ref RANGE_REGEX: Regex = Regex::new(r"(\d+):(\d+)").unwrap();
    }

    let ranges: Vec<(&str, &str)> = RANGE_REGEX
        .captures_iter(inner_span)
        .map(|capture| {
            (
                capture.get(1).map_or("", |m| m.as_str()),
                capture.get(2).map_or("", |m| m.as_str()),
            )
        }).collect();

    if ranges.is_empty() {
        // Returns the initial captured value.
        Selector::Default(String::from(inner_span))
    } else {
        // Returns the range as a tuple of the form (start,end).
        let (start, end) = &ranges[0];
        Selector::Range((
            usize::from_str_radix(start, 10).unwrap(),
            usize::from_str_radix(end, 10).unwrap(),
        ))
    }
}

/// Parse the provided selectors and returns a set of groups or an error.
pub fn selectors_parser(selectors: &str) -> Result<Groups, String> {
    match GroupsParser::parse(Rule::groups, selectors) {
        Ok(pairs) => {
            let mut groups: Groups = Vec::new();

            for pair in pairs {
                let mut group: Group = (None, Vec::new(), Vec::new());

                // Loop over the pairs converted as an iterator of the tokens
                // which composed it.
                for inner_pair in pair.into_inner() {
                    let inner_span = inner_pair.clone().into_span().as_str();

                    // Populate the group based on the rules found by the
                    // parser.
                    match inner_pair.as_rule() {
                        Rule::default => {
                            group.1.push(span_to_default(inner_span))
                        }
                        Rule::filterDefault => {
                            group.2.push(span_to_default(inner_span))
                        }
                        Rule::index => {
                            group.1.push(span_to_default(inner_span))
                        }
                        Rule::filterIndex => {
                            group.2.push(span_to_default(inner_span))
                        }
                        Rule::range => group.1.push(span_to_range(inner_span)),
                        Rule::filterRange => {
                            group.2.push(span_to_range(inner_span))
                        }
                        Rule::spread => group.0 = Some(()),
                        _ => {
                            println!("wefwef");
                            ()
                            },
                    };
                }

                // Add the group.
                groups.push(group);
            }
            Ok(groups)
        }
        Err(_) => Err(String::from("Error, unable to parse invalid selectors")),
    }
}
