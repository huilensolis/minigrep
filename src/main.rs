use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("error parsing arguments: {}", error);
        process::exit(1)
    });

    minigrep::run(&config).unwrap_or_else(|error| {
        eprintln!("error reading file: {}", error);
        process::exit(1)
    })
}
