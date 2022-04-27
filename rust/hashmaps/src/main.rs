fn main() {
    // hashmaps are like objects, maps in other languages
    //rather than getting an item by index you can get it by key
    
    // HashMap is a collection of key-value pairs
    // the key can be any type, but all keys in a hashmap must be of the same type
    // all values must also be the same type

    // example: keeping score
    // you can create hashmaps using new 

    // hashmap is the least used of the 3 collections, so it needs to be manually imported when you want to use it
    // there is less support for hashmaps, there isn't a macro to create them
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // "Blue": 10
    scores.insert(String::from("Yellow"), 50);

    // using iterators and collect 
    // there are two vectors, the teams in teams correspond to the scores in initial_scores, the indexes are the same; Yellow is 1 and 50 is 1
    let teams =  vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![String::from("10"), String::from("50")];
    // zip associates the indexes of the two vectors, associating Blue with 10 and so forth
    let mut scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    // you need to specify HashMap<_, _> so collect knows that we are collecting these values into a HashMap. the compiler can infer the key and value types, hence why they're left blank
    
    // for any values that you can copy, like u32, hashmap will copy the value
    // for strings, hashmap will take ownership of the value

    let field_name = String::from("Colour");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // we can insert references into the map, then it won't take ownership, and the values that the references point to must be valid at least as long as the hashmap is valid

    // accessing values in a hashmap

    // we can use the get method, but it returns Option<&V> so we need to handle it in some way to account for the possibility of None

    let score = match scores.get("Blue") {
        Some(score) => {println!("Blue's score is {}", score);},
        None => {println!("No score for Blue found.");},
    };

    // we can also iterate over hashmaps like we can vectors

    for (key, value) in &scores { // make sure to make a reference so this doesn't take ownership of scores
        println!("{}: {}", key, value);
    }   
    
    // updating values

    // overwriting is as simple as running map.insert.

    let mut map = HashMap::new();

    map.insert("Yes", 1);
    map.insert("Yes", 2); // overwrites the previous value
    println!("{:?}", map);

    // only inserting a value if there is no value associated with the key
    // there is a special api in hashmaps called entry, and it takes the key you want to check as a parameter. it returns an enum Entry, and it could or could not have a value
    // or_insert will return the Entry if it exists, and insert the parameter of or_insert as the value to the key (parameter of entry) if it doesn't exist

    let mut map = HashMap::new();

    map.insert("Yes", 1);

    map.entry("Yes").or_insert(3); // will not do anything because Yes already exists
    map.insert("No", 2); 
    println!("{:?}", map);


    // updating a value based on the previous value
    // this code will count the occurances of words in a string

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { // splits words at whitespace{
        let count = map.entry(word).or_insert(0); // gets the number of words if it exists and returns, or inserts 0 if it doesnt and returns. returns a mutable reference to the value, which when mutated, changes the original value it points to.
        *count += 1; // dererferences the reference to mutate it and adds 1 to the value
    }
    
    println!("{:?}", map);

}
