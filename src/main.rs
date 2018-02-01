fn main() {
    let mut s = String::from("Hello, wordl!");

    let ind = first_word(&s);

    s = String::from("wat");

    let s1 = s[0..ind];   

    println!("{}", s1);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();   // convert String to byte array

    // iterate over byte array, adding index with enumerate
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;   // index of first space
        }
    }

    s.len() // or length if no spaces (==> one word)
}
