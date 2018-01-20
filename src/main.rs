fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}   // s goes out of scope and gets dropped, so the reference points to nothing.
