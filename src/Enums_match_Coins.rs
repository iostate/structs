fn main() {
    value_in_cents(Coin::Quarter(UsState::Arizona));
    value_in_cents(Coin::Penny(UsState::Arizona));
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    // ...
}
enum Coin {
    Penny(UsState),
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny(state) => {
            println!("State penny from {:?}!", state);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
