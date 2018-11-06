use clap::{App, Arg, ArgMatches};

// Should match the Cargo.toml version!
static VERSION: &str = "0.2.1";

/// Get the CLI matches.
pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("jql")
        .about("JSON query language")
        .author("Davy Duperron <yamafaktory@gmail.com>")
        .version(VERSION)
        .arg(
            Arg::with_name("JSON")
                .help("JSON file to use")
                .index(1)
                .required(true),
        ).arg(
            Arg::with_name("selector")
                .help("Selector to apply")
                .index(2)
                .required(true),
        ).get_matches()
}
