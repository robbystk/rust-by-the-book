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

    // compute mean
    let mut sum = 0f32;
    let mut count = 0i32;
    for (number, frequency) in frequencies.iter() {
        sum += (*number * *frequency) as f32;
        count += *frequency;
    }

    let mean = sum / count as f32;

    // compute median
    let median;
    if count % 2 == 0 {
        let half_index = (count / 2) as usize;
        median = (list[half_index] + list[half_index - 1]) as f32 / 2.0;
    } else {
        let half_index = ((count - 1) / 2) as usize;
        median = list[half_index] as f32;
    };

    // compute mode
    let mut mode = 0;
    let mut max_frequency = 0;

    for (number, frequency) in frequencies.iter() {
        if *frequency > max_frequency {
            max_frequency = *frequency;
            mode = **number;
        }
    }

    println!("The mean is {}.", mean);
    println!("The median is {}.", median);
    println!("The mode is {}.", mode);
}
