use crate::types::{Group, InnerObject, Selector};

use pest::{iterators as pest_iterators, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct GroupsParser;

type PestPair<'a> = pest_iterators::Pair<'a, Rule>;

/// Convert a span to a default selector.
fn span_to_default(inner_span: &str) -> Selector {
    Selector::Default(inner_span.replace(r#"\""#, r#"""#))
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

/// Convert a span to an index selector.
fn span_to_object_index(inner_span: &str) -> InnerObject {
    if inner_span.is_empty() {
        return InnerObject::Array;
    }

    InnerObject::Index(
        inner_span
            .split(',')
            .map(|index| index.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

/// Convert a span to a range selector.
fn span_to_range(pair: PestPair<'_>) -> Selector {
    let (start, end) = pair.into_inner().fold(
        (None, None),
        |acc: (Option<PestPair<'_>>, Option<PestPair<'_>>), inner_pair| match inner_pair.as_rule() {
            Rule::start => (Some(inner_pair.clone()), acc.1),
            Rule::end => (acc.0, Some(inner_pair.clone())),
            _ => (None, None),
        },
    );

    let position_to_usize = |value: Option<PestPair<'_>>| {
        value.map(|pair| pair.as_span().as_str().parse::<usize>().unwrap())
    };

    Selector::Range((position_to_usize(start), position_to_usize(end)))
}

/// Convert a span to a range selector with an inner object.
fn span_to_object_range(pair: PestPair<'_>) -> InnerObject {
    let (start, end) = pair.into_inner().fold(
        (None, None),
        |acc: (Option<PestPair<'_>>, Option<PestPair<'_>>), inner_pair| match inner_pair.as_rule() {
            Rule::start => (Some(inner_pair.clone()), acc.1),
            Rule::end => (acc.0, Some(inner_pair.clone())),
            _ => (None, None),
        },
    );

    let position_to_usize = |value: Option<PestPair<'_>>| {
        value.map(|pair| pair.as_span().as_str().parse::<usize>().unwrap())
    };

    InnerObject::Range((position_to_usize(start), position_to_usize(end)))
}

/// Return a vector of chars found inside a default pair.
fn get_chars_from_default_pair(pair: PestPair<'_>) -> Vec<String> {
    pair.into_inner()
        .fold(Vec::new(), |mut acc: Vec<String>, inner_pair| {
            if inner_pair.as_rule() == Rule::chars {
                acc.push(String::from(inner_pair.clone().as_span().as_str()));
            }
            acc
        })
}

/// Return a vector of nested chars found inside a given pair.
fn get_inner_object_from_pair(pair: PestPair<'_>) -> Vec<InnerObject> {
    pair.into_inner()
        .fold(Vec::new(), |mut acc: Vec<InnerObject>, inner_pair| {
            match inner_pair.as_rule() {
                Rule::default => {
                    acc.push(InnerObject::Key(
                        get_chars_from_default_pair(inner_pair)[0].clone(),
                    ));
                }
                Rule::object_range => {
                    acc.push(span_to_object_range(
                        inner_pair.into_inner().next().unwrap(),
                    ));
                }
                Rule::object_index => {
                    acc.push(span_to_object_index(inner_pair.clone().as_span().as_str()));
                }
                _ => {}
            }

            acc
        })
}

/// Parse the provided selectors and returns a set of groups or an error.
pub fn selectors_parser(selectors: &str) -> Result<Vec<Group>, String> {
    match GroupsParser::parse(Rule::groups, selectors) {
        Ok(pairs) => {
            let mut groups: Vec<Group> = Vec::new();

            for pair in pairs {
                let mut group = Group::new();

                // Loop over the pairs converted as an iterator of the tokens
                // which composed it.
                for inner_pair in pair.into_inner() {
                    let inner_span = inner_pair.clone().as_span().as_str();

                    // Populate the group based on the rules found by the
                    // parser.
                    match inner_pair.as_rule() {
                        // Default
                        Rule::default => group.selectors.push(span_to_default(
                            &get_chars_from_default_pair(inner_pair)[0].clone(),
                        )),
                        Rule::filter_default => group.filters.push(span_to_default(
                            &get_chars_from_default_pair(inner_pair.into_inner().next().unwrap())
                                [0]
                            .clone(),
                        )),

                        // Index
                        Rule::index => group.selectors.push(span_to_index(inner_span)),
                        Rule::filter_index => group.filters.push(span_to_index(inner_span)),

                        // Range
                        Rule::range => group.selectors.push(span_to_range(inner_pair)),
                        Rule::filter_range => group
                            .filters
                            .push(span_to_range(inner_pair.into_inner().next().unwrap())),

                        // Property
                        Rule::property => group
                            .selectors
                            .push(Selector::Object(get_inner_object_from_pair(inner_pair))),
                        Rule::filter_property => group
                            .filters
                            .push(Selector::Object(get_inner_object_from_pair(inner_pair))),

                        // Filter lenses property.
                        Rule::filter_lens_property => group
                            .filter_lenses
                            .push(Selector::Object(get_inner_object_from_pair(inner_pair))),

                        // Root
                        Rule::root => group.root = Some(()),

                        // Spread
                        Rule::spread => group.spread = Some(()),

                        // Truncate
                        Rule::truncate => group.truncate = Some(()),
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
