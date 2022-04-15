fn main() {
     // references

     let s = String::from("           Hello world                        ");
     println!("{}", s);
     let trimmed = takes_a_ref(&s);
     println!("Here is s after running the function: {} and here is trimmed {}", s, trimmed);

     let mut s = String::from("Not Updated");
     println!("Here is the non updated string, {}", s);
     takes_ref_2(&mut s);
     println!("Inside main scope.");
     println!("Here is s inside main scope, after running secondary function with ref: {}", s);
     
}

// &String is a reference. if placed in arguments of a function, a reference won't take ownership of the variable, as the reference is a pointer to the variable's data.

fn takes_a_ref(s: &String) -> String {
    println!("This does not have to return the original value, and the string can continue to be used in the main scope.");
    println!("Here is the string inside another scope, the trimming function: {}",s);
    s.trim().to_string()
} // though s goes out of scope here, that doesn't matter because s is a reference and this function doesn't own the actual variable

// references like &ref are immutable by default, their value can't be changed.

fn takes_ref_2(s: &mut String)
{
    println!("Inside secondary function scope");
    s.push_str(", This is an update.");
}