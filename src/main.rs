use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Yellow")).or_insert(50);
    // does not insert the score of 20 because an entry for Blue already exists
    scores.entry(String::from("Blue")).or_insert(20);

    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }
}
