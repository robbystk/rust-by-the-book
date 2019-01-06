fn main() {
    let number_array = vec![34, 50, 25, 100, 65];

    let mut largest = number_array[0];

    for n in number_array {
        if n > largest {
            largest = n;
        }
    }

    println!("The largest number is {}", largest)
}
