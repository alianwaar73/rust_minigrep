use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Print out the values contained in args
    dbg!(&args);

    let (query, file_path) = parse_config(&args);

    println!("Input query: {query}\n");
    println!("Path to file: {file_path}\n");

    println!("Containing contents:\n");

    let content = fs::read_to_string(file_path)
        .expect(&format!("Should have been able to read the file specified: {file_path}"));

    println!("{content}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
