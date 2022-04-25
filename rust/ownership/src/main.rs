fn main() {
    // ints, bools, floats all have fixed sizes, but strings don't
    // at runtime, space can be reserved for fixed size variables, but not for Strings
    // fixed size stored on the stack

    
    let hi = String::from("hello");
    let hello = hi; // this is now the only variable that works, as the data was moved, not copied, for strings.
    println!("Hello world! {}", hello);

    let n1 = 1;
    let n2 = n1; // this is now a copy of n1's value, both variables work.

    println!("here is n1 {} and here is n2 {}", n1, n2);

    let hi2 = String::from("hello");
    let hello2 = hi2.clone(); // this cloned hi2, is more resource expensive, but works as copying fixed size variables would.

    println!("here is hi2 {} and here is hello2 {}", hi2, hello2);
    
    let x = 1;
    makes_copy(x);

    let s = String::from("STRING GOES HERE");
    takes_ownership(s);

    // s won't exist here

    // returning 

    let s = give_ownership();
    println!("I have been given ownership of yes, {}", s);

    let borrow = String::from("im being borrowed");

    let s = take_and_give_back(borrow);
    println!("Hi from main scope, with borrow: {}", s);

}

fn makes_copy(x: u32) {
    println!("This is a copy of x that was originally passed {}", x)
}

fn takes_ownership(s: String)
{
    println!("This is s, and it's now out of the original scope, and owned and existant in this one. {}",s)

    // s now exists here
}

fn give_ownership() -> String {
    let yes = String::from("THIS IS YOURS NOW");
    yes
}

fn take_and_give_back(s: String) -> String {
    println!("Function has ownership of borrow, but I am giving it back. Here's the value from this scope: {}", s);
    s
}