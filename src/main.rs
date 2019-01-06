fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

    println!("The largest number is {}", largest(&number_array));

    let char_array = vec!['y', 'm', 'a', 'q'];

    println!("The largest letter is {}", largest(&char_array));
}
