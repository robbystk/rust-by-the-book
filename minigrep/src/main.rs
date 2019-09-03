use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();
        Config {query, file,}
    }
}
