use std::io;

fn main() {
    println!("Enter number");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: u64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_)  => {
            println!("Please type an integer");
            return;
        },
    };

    println!("The {}th Fibonacci number is {}.", number, fib(number));
}

fn fib(n: u64) -> u64 {
    let mut next: u64;
    let mut current = 1u64;
    let mut previous = 0u64;
    for i in 1..n {
        next = current + previous;
        previous = current;
        current = next;
    }
    current
}
