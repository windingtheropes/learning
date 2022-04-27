fn main() {
    // &str is string literal and is stored in the binary itself, included in core rust, technically a string slice, is borrowed
    // String is a growable string, included in std library, is owned

    let mut s = String::new();

    let data = "initial contents";

    // converting a string literal to String
    let s = data.to_string();

    // or 
    let s = "initial contents".to_string();

    //or using String::from()
    let s = String::from("initial contents");

    // pushing data to a string

    let mut s = String::from("foo");
    s.push_str("bar");
    // s is now foobar

    // push_str takes a string slice because it doesnt make sense for it to take ownership, so we can do this too

    let s2 = "bar";

    s.push_str(s2); 
    // s is now foobarbar
    println!("s2 is {}", s2);
    // if push_str took ownership we wouldn't be able to use s2 here

    // pushing a char to a string

    s.push('l');
    // s is now foobarbarl

    // using the + operator

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used
    // the reason it takes ownership of s1 is because the add method takes (self, s: &str), self being s1 and s being s2 in this case
    // we can only add an &str to a string, not another String, that's why the second half is a reference
    // even though &s2 is an &String, the compiler doesnt throw an error because it can coerce &String to &str
    println!("s3 is {}", s3);

    // adding multiple strings together
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // while this works, the line gets long and hard to understand what is going on
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("s4 is {}", s4);

    // using format! to add multiple strings together
    // this method also doesn't take ownership of anything, so s1, s2, and s3 all remain valid
    // format is a macro and it uses references
    // this is overall easier to read
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s4 is {}", s4);


    // indexing strings
    // this string's length is 4
    let hello = String::from("hola");

    // this string's length is 24, not 12, becasue the length is the amount of bytes it takes to encode the text
    let hello = "Здравствуйте";

    // rust does not allow indexing Strings

    // instead, you can slice strings, which we already looked at 

    let hello = "hola";
    let s = &hello[0..1]; // this is a string slice, which is a reference to a part of a string
    // an ampersand prefixing a string can also be called a string slice, because it's referencing to a part of a string, and it iself doesn't have a string value
    println!("s is {}", s);

    // iterating over strings

    // with characters
    for c in hello.chars() {
        println!("{}", c);
    }

    // with bytes
    for b in hello.bytes() {
        println!("{}", b);
    }
    
}
