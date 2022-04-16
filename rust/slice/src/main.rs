fn main() {
    let s = String::from("Hello world!");

    let hello = &s[0..5];
    let world = &s[6..12]; // bottom inclusive top exclusive
                           // 6..12 = 6,7,8,9,10,11

    println!("{} {}", hello, world);

    let fw = first_word(&s);
    println!("first word is {}", fw);

    // the slice is a pointer to the original string s, so if s is cleared the slice will point to nothing and throw an error
    
    let options = String::from("fjhks");

    let ob = options.as_bytes();

    for (i, &item) in ob.iter().enumerate() {
        let prev_index = if i as i32 - 1 > -1 { i } else { 0 };
        println!("{}", &options[prev_index..i+1]);
    }
}

fn first_word(s: &str) -> &str { // &str allows string literals and String references.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        } 
    }
    &s[..]
}