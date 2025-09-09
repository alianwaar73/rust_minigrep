use std::error::Error;
use std::fs;

// Public structure for CLI configuration
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Build configuration from CLI args with basic validation
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough input arguments provided.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// Core program logic; prints file contents for now
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!("{content}");
    Ok(())
}

// Taking the TDD: Test-driven development
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Pick three."], search(query, contents));
    }
}

// [ ] <'a> in the below defines the lifecyle
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
