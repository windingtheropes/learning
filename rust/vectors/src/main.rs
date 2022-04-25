fn main() {
    //vectors are growable arrays, with items all of the same type
    // we need to specify a type if we initialize it as empty
    let v: Vec<i32> = Vec::new();
    // most often we'll initialize vecotrs with a elements already, and rust will automatically infer the type inside
    
    let mut v = vec![1, 2, 3];

    //updating a vector, the vector must be mutable
    v.push(5);
    v.push(6);

    let third: &i32 = &v[2]; // reference so we don't take ownership of v
    println!("The third element is {}", third);

    // let does_not_exist = &v[100]; // the program will crash because index 100 doesn't exist
    // let does_not_exist = v.get(100); // the program will return None and the program will not crash, can be used in match like below. if there was index 100, it would return Some(&element)

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // the 3 lines below won't work because you cannot have mutable and immutable references in the same scope

    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    // iterating through a vector

    let v1 = vec![1,2,3,4,5];

    for i in &v {
        // make sure to get a reference so we don't take ownership
        println!("{}", i);
    }

    let mut v2 = vec![1,2,3,4,5];

    // mutable iterating through a vector
    for i in &mut v2 {
        // make sure to get a reference so we don't take ownership
        // *i gets the mutable reference to the element in the vector
        *i += 1; // changing each element to be increased by 1
    }

    println!("v2 is: {:?}", v2);

    // items in a vector need to be all the same type, but sometimes you may want to store different types of values. we can use enums

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // the enum variants will be considered the same type, that of the enum itself. 
    let row = vec![
        SpreadsheetCell::Int(3), // enum
        SpreadsheetCell::Text(String::from("blue")), // enum
        SpreadsheetCell::Float(10.12), // enum
    ];

    // let will_not_work = vec!
    // [1, // int 
    // "2", // string
    // 3.0, // float] 
}

// the vector leaves scope here, and all values are dropped with it
