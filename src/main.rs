fn largest(list: &[i32]) -> i32 {
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

    let number_array = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("The largest number is {}", largest(&number_array));
}
