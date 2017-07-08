use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Get a number of spaces from user
    println!("Enter your desired number of spaces");
    let mut spaces = String::new();
    io::stdin().read_line(&mut spaces)
        .expect("Failed to read line");

    let spaces = spaces.len();

    println!("Number of spaces: {}", spaces);
}
