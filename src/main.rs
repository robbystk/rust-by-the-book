fn main() {
    let _reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s   // return s so its ownership transfers to caller
}
