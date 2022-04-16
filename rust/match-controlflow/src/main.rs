#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    California,
    NewYork
}

enum Coin {
    Penny, 
    Nickel, 
    Dime, 
    Quarter(UsState)
}

fn main() {
    let coin = Coin::Quarter(UsState::NewYork);

    let value = value_in_cents(coin);

    println!("{}", value)


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let roll = 0;

    match roll {
        1 =>, // do stuff with a roll of 1
        2 =>, // do stuff with a roll of 2
        other =>, // do stuff for anything else, other is equal to the value. if we used _, it wouldn't have a value. this could be used if in the game you wanted to reroll if you got anything but a 1 or 2, because you don't need the value of the dice that isnt 1 or 2 at any time.
        // _ => (), does nothing when a roll other than 1 or 2 appears. but it handles the remainder of rolls that we need to handle. remember _ has no value but it represents the unhandled possibilities.
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            1
        },
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("A quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // the none case MUST be handled
        Some(i) => Some(i + 1), // not sure why returning Some(i+1) and not i+1
    }
}
