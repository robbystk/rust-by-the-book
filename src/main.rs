fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];  // borrow reference to third element

    v.push(6);
}
