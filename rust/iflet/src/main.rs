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
    // if let is used for situations where you only want to execude code under one pattern condition

    //using match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the max is {}", max),
        _ => (), // if let will eliminate this boilerplate
    };

    //using if let

    if let Some(max) = config_max { // this code will only run if the value matches the pattern
        println!("the max is {}", max);
    }
    // or we could add an else, which is the same as the _ => //code in a match statement.

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("the state is {:?}", state)
    }
    else {
        count += 1;
    }
    println!("counted {} other coin", count)


}
