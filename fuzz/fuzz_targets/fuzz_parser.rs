#![no_main]

extern crate libfuzzer_sys;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = jql_parser::parser::parse(s);
    }
});
