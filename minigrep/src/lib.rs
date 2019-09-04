use std::error::Error;
use std::fs;
use std::env;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file)?;

    let results = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        case_insensitive_search(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, file, case_sensitive,})
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents),
        );
    }

    #[test]
    fn test_case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:",
                "Trust me."],
            case_insensitive_search(query, contents),
        );
    }
}
