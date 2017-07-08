fn main() {
    // create a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructure to extract element
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // can also directly access
    let one = tup.2;
    println!("The third element of tup is: {}", one);
}
