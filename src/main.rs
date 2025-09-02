use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Print out the values contained in args
    dbg!(&args);

    let config = Config::new(&args);

    println!("Input query: {}", config.query);
    println!("Path to file: {}", config.file_path);
    println!("Containing contents:\n");

    let content = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file specified.");

    println!("{content}");
}

// Structure for Config type
struct Config {
    query: String,
    file_path: String,
}

// Creating the constructor for CLI argument parsing 
// Config
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
