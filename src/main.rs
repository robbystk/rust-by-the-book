fn main() {
    let condition = true;
    let number = if condition {
        5   // note lack of semicolons
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
