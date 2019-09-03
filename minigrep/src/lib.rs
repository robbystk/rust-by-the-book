use std::error::Error;
use std::fs;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.file)?;

    for line in search(&cfg.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        Ok(Config {query, file,})
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
}
