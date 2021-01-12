use regex::Regex;
use std::error::Error;
use std::fs;
use std::{env, process};

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Please provide at least two arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        let mut options = None;
        if args.len() >= 4 {
            options = Some(&args[3..]);
        }

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        if let Some(i) = options {
            for option in i {
                match option.as_str() {
                    "-h" => {
                        println!("-i for case insensitive, -h to print this message again.");
                        process::exit(0);
                    }
                    "-i" => {
                        case_sensitive = false
                    }
                    _ => {
                        println!(
                            "`{}` is not a known flag, try -h to search for commands.",
                            option
                        )
                    }
                }
            }
        }

        let config = Config {
            query: String::from(query),
            filename: String::from(filename),
            case_sensitive: case_sensitive,
        };

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results.iter() {
        println!("{}", line);
    }
    Ok(())
}

// The lifetime is basically saying that the output is going to live at least as long as the contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Go through content line
    // Match query in line
    // If yes, return, keep going
    // Return nothign

    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search_case_insensitive_regex<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    // Compile search res insensitive
    let reg_query = Regex::new(&format!("(?i){}", query)).unwrap();
    for line in contents.lines() {
        if reg_query.is_match(line) {
            results.push(line.trim());
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    let query = query.to_lowercase();

    // Compile search res insensitive
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.trim());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_default() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive_regex() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive_regex(query, contents)
        );
    }
}
