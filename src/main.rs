fn main() {
    let some_u8_value = Some(7u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
