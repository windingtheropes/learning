fn main() {
     // references

     let s = String::from("           Hello world                        ");
     println!("{}", s);
     let trimmed = takes_a_ref(&s);
     println!("Here is s after running the function: {} and here is trimmed {}", s, trimmed);

     
}

// &String is a reference. if placed in arguments of a function, a reference won't take ownership of the variable, as the reference is a pointer to the variable's data.

fn takes_a_ref(s: &String) -> String {
    println!("This does not have to return the original value, and the string can continue to be used in the main scope.");
    println!("Here is the string inside another scope, the trimming function: {}",s);
    s.trim().to_string()
}