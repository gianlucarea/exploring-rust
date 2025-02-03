#[derive(Debug)]
enum UsState{
    Alabama,
    NewYork,
    Florida,
    Ohio,
    Texas,
    Delaware,
}

enum Coin{
    Penny,
    Nickel,
    Dime(UsState),
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Dime(UsState::Alabama);
    let value = value_in_cents(&coin);
    
    // let coin = Coin::Penny;
    println!("The value of the coin is {value}");

    let mut count = 0;
    if let Coin::Dime(state) = coin {
         println!("State dime from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count value of no dime is {count}");
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime(_state) => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        }
    }
}