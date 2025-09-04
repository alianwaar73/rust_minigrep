use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Uncomment the following flag to print out the values contained in args
    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing input arguments: {err}");
        process::exit(1);
    });

    println!("Input query: {}", config.query);
    println!("Path to file: {}", config.file_path);
    println!("Containing contents:\n");

    if let Err(e) = minigrep::run(config) {
        println!("Application errored: {e}");
        process::exit(1);
    }
}


