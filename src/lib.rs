use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub show_line_numbers: bool,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, String> {
        Self::build_with_env(args, env::var("IGNORE_CASE").is_ok())
    }

    fn build_with_env(
        mut args: impl Iterator<Item = String>,
        mut ignore_case: bool,
    ) -> Result<Config, String> {
        let _program_name = args.next();

        let mut show_line_numbers = false;
        let mut query: Option<String> = None;
        let mut file_path: Option<String> = None;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--" => {
                    for positional in args {
                        if query.is_none() {
                            query = Some(positional);
                        } else if file_path.is_none() {
                            file_path = Some(positional);
                        } else {
                            return Err(format!(
                                "Unexpected extra argument '{}'.\n\n{}",
                                positional,
                                Self::usage()
                            ));
                        }
                    }
                    break;
                }
                "-i" | "--ignore-case" => {
                    ignore_case = true;
                }
                "-I" | "--no-ignore-case" => {
                    ignore_case = false;
                }
                "-n" | "--line-number" | "--line-numbers" => {
                    show_line_numbers = true;
                }
                "-h" | "--help" => {
                    return Err(Self::usage());
                }
                _ => {
                    if query.is_none() {
                        query = Some(arg);
                    } else if file_path.is_none() {
                        file_path = Some(arg);
                    } else {
                        return Err(format!(
                            "Unexpected extra argument '{}'.\n\n{}",
                            arg,
                            Self::usage()
                        ));
                    }
                }
            }
        }

        let query = query.ok_or_else(|| format!("Missing search query.\n\n{}", Self::usage()))?;
        let file_path =
            file_path.ok_or_else(|| format!("Missing file path.\n\n{}", Self::usage()))?;

        Ok(Config {
            query,
            file_path,
            ignore_case,
            show_line_numbers,
        })
    }

    pub fn usage() -> String {
        "Usage: minigrep [OPTIONS] <query> <file_path>\n\nOPTIONS:\n    -i, --ignore-case       Search without case sensitivity or when IGNORE_CASE is set.\n    -I, --no-ignore-case    Force case-sensitive search even if IGNORE_CASE is set.\n    -n, --line-number       Prefix matches with their line number.\n".to_string()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = search_with_config(&config.query, &contents, config.ignore_case);
    let query_lower = config.ignore_case.then(|| config.query.to_lowercase());

    for result in results {
        let highlighted = highlight_line(
            result.line,
            &config.query,
            config.ignore_case,
            query_lower.as_deref(),
        );

        if config.show_line_numbers {
            println!("{}: {}", result.line_number, highlighted);
        } else {
            println!("{}", highlighted);
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub struct Match<'a> {
    pub line_number: usize,
    pub line: &'a str,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search_with_config(query, contents, false)
        .into_iter()
        .map(|hit| hit.line)
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search_with_config(query, contents, true)
        .into_iter()
        .map(|hit| hit.line)
        .collect()
}

pub fn search_with_config<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<Match<'a>> {
    let mut results = Vec::new();
    let query_lower = ignore_case.then(|| query.to_lowercase());

    for (idx, line) in contents.lines().enumerate() {
        let is_match = if let Some(lower_query) = query_lower.as_ref() {
            line.to_lowercase().contains(lower_query)
        } else {
            line.contains(query)
        };

        if is_match {
            results.push(Match {
                line_number: idx + 1,
                line,
            });
        }
    }

    results
}

const ANSI_BOLD: &str = "\u{1b}[1m";
const ANSI_RESET: &str = "\u{1b}[0m";

fn highlight_line(line: &str, query: &str, ignore_case: bool, query_lower: Option<&str>) -> String {
    if query.is_empty() {
        return line.to_string();
    }

    if ignore_case {
        if let Some(lower_query) = query_lower {
            return highlight_case_insensitive(line, lower_query);
        }

        let lower_query_owned = query.to_lowercase();
        return highlight_case_insensitive(line, lower_query_owned.as_str());
    }

    highlight_case_sensitive(line, query)
}

fn highlight_case_sensitive(line: &str, query: &str) -> String {
    let mut highlighted = String::with_capacity(line.len());
    let mut last_idx = 0;

    for (idx, _) in line.match_indices(query) {
        highlighted.push_str(&line[last_idx..idx]);
        highlighted.push_str(ANSI_BOLD);
        highlighted.push_str(&line[idx..idx + query.len()]);
        highlighted.push_str(ANSI_RESET);
        last_idx = idx + query.len();
    }

    highlighted.push_str(&line[last_idx..]);
    highlighted
}

fn highlight_case_insensitive(line: &str, query_lower: &str) -> String {
    // For this learning project we assume ASCII input so byte offsets align after lowercasing.
    let mut highlighted = String::with_capacity(line.len());
    let lower_line = line.to_lowercase();

    let mut last_idx = 0;
    let mut search_idx = 0;

    while let Some(found) = lower_line[search_idx..].find(query_lower) {
        let match_start = search_idx + found;
        let match_end = match_start + query_lower.len();

        highlighted.push_str(&line[last_idx..match_start]);
        highlighted.push_str(ANSI_BOLD);
        highlighted.push_str(&line[match_start..match_end]);
        highlighted.push_str(ANSI_RESET);

        last_idx = match_end;
        search_idx = match_end;
    }

    highlighted.push_str(&line[last_idx..]);
    highlighted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "ick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["Pick three."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn line_numbers() {
        let query = "Pick";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let matches = search_with_config(query, contents, false);
        assert_eq!(matches.len(), 1);
        let hit = &matches[0];
        assert_eq!(hit.line_number, 3);
        assert_eq!(hit.line, "Pick three.");
    }

    #[test]
    fn config_parses_flags() {
        let args = vec![
            "minigrep",
            "--ignore-case",
            "--line-number",
            "pattern",
            "file.txt",
        ]
        .into_iter()
        .map(String::from);

        let config = Config::build_with_env(args, false).expect("should parse flags");
        assert!(config.ignore_case);
        assert!(config.show_line_numbers);
        assert_eq!(config.query, "pattern");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn highlight_case_sensitive_output() {
        let line = "Pick three.";
        let query = "Pick";

        let highlighted = highlight_line(line, query, false, None);
        assert_eq!(
            highlighted,
            format!("{}Pick{} three.", super::ANSI_BOLD, super::ANSI_RESET)
        );
    }

    #[test]
    fn highlight_case_insensitive_output() {
        let line = "Rust is trustworthy";
        let query = "rust";
        let highlighted = highlight_line(line, query, true, Some("rust"));

        assert_eq!(
            highlighted,
            format!(
                "{}Rust{} is t{}rust{}worthy",
                super::ANSI_BOLD,
                super::ANSI_RESET,
                super::ANSI_BOLD,
                super::ANSI_RESET
            )
        );
    }
}
