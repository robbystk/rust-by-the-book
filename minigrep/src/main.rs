use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong while reading the file");

    println!("With text:\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let filename = &args[2];
    Config { query, filename }
}