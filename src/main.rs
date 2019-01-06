// Rust can't tell whether the returned reference will be from s1 or s2 (and
// neither can we), so we have to specify the lifetime
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("The longest string is {}", result);
}
