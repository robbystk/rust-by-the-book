fn main() {
    let s1 = String::from("Hello,");
    let s2 = String::from(" world!");

    let s3 = s1 + &s2;  // s1 gets moved here (to s3, kind of)
    // s2 must be borrowed to avoid taking ownership of it and because that's
    // what the add() function expects.

    // The function signature of String::add() is approximately
    // fn add(self, s: &str) -> String {

    // Note even though the signature asks for a &str, we give it an &String
    // which gets automatically coerced into a string slice.

    // so we can no longer use s1 (but can still use s2)
    // println!("{}", s1);

    println!("{}", s3);
}
