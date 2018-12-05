use clap::{App, Arg, ArgMatches};

/// Get the CLI matches.
pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("jql")
        .about("JSON Query Language")
        .author("Davy Duperron <yamafaktory@gmail.com>")
        .version(crate_version!())
        .arg(
            Arg::with_name("JSON")
                .help("JSON file to use")
                .index(1)
                .required(true),
        ).arg(
            Arg::with_name("selectors")
                .help("Selectors to apply")
                .index(2)
                .required(true),
        ).arg(
            Arg::with_name("inline")
                .help("Inlines JSON output")
                .long("inline")
                .short("i"),
        ).get_matches()
}
