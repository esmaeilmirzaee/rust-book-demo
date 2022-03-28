use std::env;
use std::error::Error;
use std::ops::Add;
use std::fs;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_term_one_result() {
        let term = "duct";
        let contents = "\
Rust:
Safe, Fast, Productive.
Pick three.";
        assert_eq!(vec!["Safe, Fast, Productive."], search(term, contents));
    }
}
