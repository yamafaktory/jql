use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg,
    ArgMatches,
};

/// Get the CLI matches.
pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("selectors")
                .help("Selectors to apply")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("JSON")
                .help("JSON file to use")
                .index(2)
                .required(false),
        )
        .arg(
            Arg::with_name("inline")
                .help("Inlines JSON output")
                .long("inline")
                .short("i"),
        )
        .arg(
            Arg::with_name("raw-output")
                .help("Writes raw string selection directly to standard output without JSON double-quotes")
                .long("raw-output")
                .short("r"),
        )
        .get_matches()
}
