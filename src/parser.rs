use pest::Parser;
use types::{Group, Groups};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GroupsParser;

pub fn selectors_parser(selectors: &str) -> Result<Groups, String> {
    match GroupsParser::parse(Rule::groups, selectors) {
        Ok(pairs) => {
            let mut groups: Groups = Vec::new();

            for pair in pairs {
                let mut current_group: Group = (None, Vec::new(), Vec::new());

                // Loop over the pairs converted as an iterator of the tokens
                // which composed it.
                for inner_pair in pair.into_inner() {
                    let stringified_inner_span =
                        inner_pair.clone().into_span().as_str().to_string();

                    // Populate the group based on the rules found by the
                    // parser.
                    match inner_pair.as_rule() {
                        Rule::filter => {
                            current_group.2.push(stringified_inner_span)
                        }
                        Rule::selector => {
                            current_group.1.push(stringified_inner_span)
                        }
                        Rule::spread => {
                            current_group.0 = Some(stringified_inner_span)
                        }
                        _ => (),
                    };
                }
                // Add the group.
                groups.push(current_group.clone());
            }
            Ok(groups)
        }
        Err(_) => Err(String::from("Error, unable to parse invalid selectors")),
    }
}
