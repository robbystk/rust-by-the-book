fn main() {
    // Array (allocated on stack) (fixed size)
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let third = a[2];

    println!("The third element of a is: {}", third);

    // runtime error with out of bound index
    println!("The eleventh element f a is: {}", a[index]);
}
