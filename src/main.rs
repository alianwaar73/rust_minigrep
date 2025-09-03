use std::env;
use std::fs;
use std::process;

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

    let content = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file specified.");

    println!("{content}");
}

// Structure for Config type
struct Config {
    query: String,
    file_path: String,
}

// Creating the constructor Config for CLI argument parsing 
impl Config {

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough input arguments provided.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
