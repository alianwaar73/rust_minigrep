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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_rejects_missing_args() {
        let args = vec!["bin".to_string()];
        assert!(Config::build(&args).is_err());
    }
}

