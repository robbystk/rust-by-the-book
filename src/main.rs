enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    // etc
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(Some(state)) => {
            println!("{:?} state quarter!", state);
            25
        },
        Coin::Quarter(None) => 25,
    }
}

fn main() {
    let state_quarter = Coin::Quarter(Some(UsState::Colorado));
    let stateless_quarter = Coin::Quarter(None);

    println!("The coin has a value of {} cents",
             value_in_cents(state_quarter));
    println!("The coin has a value of {} cents",
             value_in_cents(stateless_quarter));
}
