use minigrep;
use minigrep::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem is: {err}");
        process::exit(1);
    });
    println!(
        "Search '{}' in file {} with ignore-case={}",
        config.query, config.file_path, config.ignore_case
    );

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
