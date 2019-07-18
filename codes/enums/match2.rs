enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(USState)
}

#[derive(Debug)]
enum USState{
    Alabama,
    Alaska
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main(){
    let value = value_in_cents(Coin::Quarter(USState::Alaska));
    println!("The value of value is:{}", value);
}
