use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 20);    // clobbers existing score

    println!("The Blue team has a score of {}", scores.get("Blue").unwrap())
}
