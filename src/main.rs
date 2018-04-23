fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    // compile-time error:
    // let guess = "42".parse().expect("Not a number!");
}
