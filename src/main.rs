fn main() {
    let s = String::from("Hello world");

    let _hello = &s[..5];
    let _world = &s[6..];

    let first = first_word(&s);

    println!("{}", first);
    
    let s2 = first_word("literal strings work too");
    println!("{}", s2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();   // convert String to byte array

    // iterate over byte array, adding index with enumerate
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];   // slice of first word
        }
    }

    &s[..] // or entire string if no spaces (==> one word)
}
