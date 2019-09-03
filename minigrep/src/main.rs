use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file);

    let contents = fs::read_to_string(config.file)
        .expect("Something went wrong while reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file = args[2].clone();
    Config {query, file,}
}
