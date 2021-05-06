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
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough params! at least two params are required.");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut lines_found = vec![];

    for line in contents.lines() {
        let mut query_to_search = query;
        let query_lowercase = &query.to_lowercase();

        // compare lower case string
        if !case_sensitive {
            query_to_search = query_lowercase;
        }

        if line.contains(query_to_search) {
            lines_found.push(line.trim());
        }
    }
    lines_found
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
