fn main() {
    let y = 6;  // this is a statement (no return)
    // no worky:
    // let wat = (let y = 6);

    y + 1;  // y + 1 is an expression (returns)

    // RHS of this is an expression that evaluates to 4
    let thing = {
        let x = 3;
        x + 1
    }

    // a semicolon turns an expression into a statement

    another_function(5, 6); // also an expression (except semicolon)
}

// function definition is a statement
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);   // expression (except semicolon)
    println!("The value of y is: {}", y);
}
