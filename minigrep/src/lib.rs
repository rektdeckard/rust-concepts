//! # minigrep
//!
//! A naive implementation of grep-like search that finds instances
//! of a string of text within a target file, on case-sensitive and
//! insensitive basis.

use std::env;
use std::error::Error;
use std::fs;

/// Config arguments
///
/// Indicate configuration parameters for searching text. Arguments are
/// positional, and currently the case-sensitivity flag must be set with
/// an environment variable, though this should be updated to take a
/// command-line flag.
///
/// # Examples
///
/// ```
/// let config = minigrep::Config {
///     query: "the".to_string(),
///     filename: "poem.txt".to_string(), // path is relative to project root
///     case_sensitive: true,
/// };
///
/// assert_eq!(config.filename, "poem.txt");
/// ```
///
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<I: Iterator<Item = String>>(mut args: I) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_constructor_works() {
        let args = vec![
            String::from("test"),
            String::from("the"),
            String::from("poem.txt"),
        ];
        let config = Config::new(args.into_iter()).unwrap();
        assert_eq!(config.query, "the");
        assert_eq!(config.filename, "poem.txt");
    }

    #[test]
    fn config_constructor_fails_with_bad_args() {
        let args = vec![String::from("too"), String::from("few")];
        let config = Config::new(args.into_iter());
        assert!(config.is_err());
        assert_eq!(config.err(), Some("Didn't get a file name"));
    }

    #[test]
    fn run_can_read_file() {
        let config = Config {
            query: String::from("the"),
            filename: String::from("poem.txt"),
            case_sensitive: true,
        };
        assert!(run(config).is_ok());
    }

    #[test]
    fn run_fails_with_bad_file() {
        let config = Config {
            query: String::from("fail"),
            filename: String::from("fail.txt"),
            case_sensitive: true,
        };
        assert!(run(config).is_err());
    }

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
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
}
