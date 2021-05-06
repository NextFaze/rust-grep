//! # Rust Grep
//!
//! `rust_grep` is a collection of utilities to find texts from a file

use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results = search(&config.query, &contents, config.case_sensitive);

    for result in results {
        println!("{}", result);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough params! at least two params are required.");
        }
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing required search query string"),
        };

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing required file name to run query search on."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

/// Searches for given query in the specified file
/// # Examples
/// ```
/// let query = "Productive";
/// let contents = "\
/// Rust,
/// Safe, fast and productive
/// Pick all three.
/// ";
///
/// assert_eq!(vec![] as Vec<String>, search_query::search(query, contents, true));
/// ```
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut query_to_search = query;
    let query = &query.to_lowercase();
    if !case_sensitive {
        query_to_search = query;
    }
    contents
        .lines()
        .filter(|line| line.contains(query_to_search))
        .map(|line| line.trim())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_query() {
        let query = "duct";
        let contents = "\
        Rust,
        Safe, fast and productive
        Pick all three.
        ";

        assert_eq!(
            vec!["Safe, fast and productive"],
            search(query, contents, false)
        );
    }

    #[test]
    fn search_query_insensitive() {
        let query = "Productive";
        let contents = "\
        Rust,
        Safe, fast and productive
        Pick all three.
        ";

        assert_eq!(vec![] as Vec<String>, search(query, contents, true));
    }
}
