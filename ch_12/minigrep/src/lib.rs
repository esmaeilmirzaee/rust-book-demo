use std::env;
use std::error::Error;
use std::fs;
use std::ops::Add;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Invalid arguments input, please provide `query` and `filename`");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: &mut Config) -> Result<(), Box<dyn Error>> {
    if !config.filename.contains(".txt") {
        config.filename = config.filename.clone().add(".txt");
    }

    let content = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    for line in search_case_insensitive("TOO", &content) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(term: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&term) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    println!("{:?}", results);

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.";
        assert_eq!(vec!["Safe, Fast, Productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.
Duct tape.";

        assert_ne!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
