
fn main() {
    // when code panics there is no way to recover
    // in tests, panics are a smarter idea because they lead directly to the bug
    // panics are basically the program giving up and the programmer aknowledging that they're not going to do anything to try and handle the error
    // panics are also a good way in prototyping to show the error you're receiving to help decide better how to handle it
    // in any case that you're 100% sure there cannot be an error, you can safely call unwrap without the worry of your program panicking
    // an example of the above is hardcoded values

    // if datais passed that you refuse to read, panic is a good idea. examples could be passing the wrong data type
    // in an api or library it is deffinitely smart to use panic wherever you can for errors as it helps the programmer see the issues with implementation and take steps to fix them

    // if failure is expected you should use result, and handle the error however you want, continuing operation

    // sometimes error checking can get repetitive, so you can use rust's type checker when applicable, and it will make sure that the values you are working with are always matching the initial set type of the variable
    
}

// going back to the guessing game; if you wanted to ensure at every point that the guess was between 1 and 100, you would have to continuously check it
    // in this case you could make a custom type that can only be between 1 and 100, and then use that type for the guess

    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
    
            Guess { value }
        }
    }
    