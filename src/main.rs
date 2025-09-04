use std::env;
use std::fs;
use std::process;
use std::error::Error;

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

    if let Err(e) = run(config) {
        println!("Application errored: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.file_path)?;

    println!("{content}");

    Ok(())
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
