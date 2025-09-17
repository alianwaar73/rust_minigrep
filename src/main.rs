use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing input arguments: {err}");
        process::exit(1);
    });

    println!("Input query: {}", config.query);
    println!("Path to file: {}", config.file_path);
    if config.ignore_case {
        println!("Case-insensitive search enabled");
    }
    if config.show_line_numbers {
        println!("Line numbers will be printed");
    }

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application errored: {e}");
        process::exit(1);
    }
}
