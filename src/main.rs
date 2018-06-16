use std::collections::HashMap;

fn main() {
    let field_name = String::from("My favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{}", field_name); // Can't because it's owned by map now.
}
