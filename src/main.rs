fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn main() {
    let number_array = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest_i32(&number_array));

    let char_array = vec!['y', 'm', 'a', 'q'];

    println!("The largest letter is {}", largest_char(&char_array));
}
