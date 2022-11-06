#[derive(Debug)] // so we can inspect the state in a minute

enum UsState {
    NewYork,
    California
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, Control Flow!");
    let penny = Coin::Penny;
    let result = value_in_cents(penny);
    println!("The result was {} penny", result);

    let quarter = Coin::Quarter(UsState::California);
    let result = value_in_cents(quarter);
    println!("The result was {} Quarter", result);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}