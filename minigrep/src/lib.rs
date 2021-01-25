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
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        // Ignore first param, which is just the executable
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // Iterate through remaining arguments as options
        for option in args {
            match option.as_str() {
                "-h" => {
                    println!("-i for case insensitive, -h to print this message again.");
                    process::exit(0);
                }
                "-i" => case_sensitive = false,
                _ => {
                    println!(
                        "`{}` is not a known flag, try -h to search for commands.",
                        option
                    )
                }
            }
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line.trim());
    //     }
    // }
    
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive_regex<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     if reg_query.is_match(line) {
    //         results.push(line.trim());
    //     }
    // }
    // results

    // Compile search res insensitive
    let reg_query = Regex::new(&format!("(?i){}", query)).unwrap();
    contents.lines().map(|line| line.trim()).filter(|line| reg_query.is_match(line)).collect()
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
