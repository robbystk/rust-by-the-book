use std::collections::HashMap;

fn main() {
    let mut list = [1, 5, 6, 4, 8, 9, 10, 3, 11, 2, 1, 1, 9, 5, 4, 5];

    // we do not give a shit about stability, and unstable sorting is faster and
    // uses less memory.
    list.sort_unstable();

    // hash map to store values as frequencies
    let mut frequencies = HashMap::new();

    for number in list.iter() {
        let count = frequencies.entry(number).or_insert(0);
        *count += 1;
    };

    for (key, value) in frequencies.iter() {
        println!("{}: {}", key, value);
    }

    // compute mean
    let mut sum = 0f32;
    let mut count = 0f32;
    for (number, frequency) in frequencies.iter() {
        sum += (*number * *frequency) as f32;
        count += *frequency as f32;
    }

    let mean = sum / count;

    println!("The mean is {}.", mean);
}
