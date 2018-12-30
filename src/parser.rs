use crate::types::Selector;
use crate::types::{Group, Groups};
use lazy_static::lazy_static;
use pest::{iterators as pest_iterators, Parser};
use pest_derive::*;
use regex::Regex;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroupsParser;

/// Convert a span to a default selector.
fn span_to_default(inner_span: String) -> Selector {
    Selector::Default(inner_span)
}

/// Convert a span to an index selector.
fn span_to_index(inner_span: &str) -> Selector {
    if inner_span.is_empty() {
        return Selector::Array;
    }

    Selector::Index(
        inner_span
            .split(',')
            .map(|index| index.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

/// Convert a span to an object selector.
fn span_to_object(inner_span: Vec<String>) -> Selector {
    Selector::Object(inner_span)
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
        })
        .collect();

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

/// Return a vector of chars found inside a default pair.
fn get_chars_from_default_pair(
    pair: pest_iterators::Pair<'_, Rule>,
) -> Vec<String> {
    pair.into_inner()
        .fold(Vec::new(), |mut acc: Vec<String>, inner_pair| {
            if inner_pair.as_rule() == Rule::char {
                acc.push(String::from(inner_pair.clone().as_span().as_str()));
            }
            acc
        })
}

/// Return a vector of nested chars found inside a given pair.
fn get_nested_chars_from_default_pair(
    pair: pest_iterators::Pair<'_, Rule>,
) -> Vec<String> {
    pair.into_inner()
        .fold(Vec::new(), |mut acc: Vec<Vec<String>>, inner_pair| {
            if inner_pair.as_rule() == Rule::default {
                acc.push(get_chars_from_default_pair(inner_pair));
            }
            acc
        })
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
}

/// Parse the provided selectors and returns a set of groups or an error.
pub fn selectors_parser(selectors: &str) -> Result<Groups, String> {
    match GroupsParser::parse(Rule::groups, selectors) {
        Ok(pairs) => {
            let mut groups: Groups = Vec::new();

            for pair in pairs {
                let mut group: Group = (None, None, Vec::new(), Vec::new());

                // Loop over the pairs converted as an iterator of the tokens
                // which composed it.
                for inner_pair in pair.into_inner() {
                    let inner_span = inner_pair.clone().as_span().as_str();

                    // Populate the group based on the rules found by the
                    // parser.
                    match inner_pair.as_rule() {
                        Rule::default => group.2.push(span_to_default(
                            get_chars_from_default_pair(inner_pair)[0].clone(),
                        )),
                        Rule::filter_default => group.3.push(span_to_default(
                            get_chars_from_default_pair(inner_pair)[0].clone(),
                        )),
                        Rule::index => group.2.push(span_to_index(inner_span)),
                        Rule::filter_index => {
                            group.3.push(span_to_index(inner_span))
                        }
                        Rule::range => group.2.push(span_to_range(inner_span)),
                        Rule::filter_range => {
                            group.3.push(span_to_range(inner_span))
                        }
                        Rule::property => group.2.push(span_to_object(
                            get_nested_chars_from_default_pair(inner_pair),
                        )),
                        Rule::root => group.1 = Some(()),
                        Rule::spread => group.0 = Some(()),
                        _ => (),
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
