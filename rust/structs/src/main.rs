struct User {  // names in this struct
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Colour(i32, i32, i32); // tuple struct, no names

struct AlwaysEqual; // struct with no fields, can be used for traits. no data stored in this.

// Values don't need to be in order to be assigned.

fn main() {
    let mut user1 = User {
        email: String::from("test@example.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };

    // we can change values if the struct is mutable
    // if the entire struct is immutable, nothing inside can be mutable, and similarly if a struct is mutable all parts are mutable

    user1.email = String::from("another@example.com");

    println!("{}'s email is {}", user1.username, user1.email);

    let user2 = build_user(String::from("test2@example.com"), String::from("test2"));
    println!("{}'s email is {}", user2.username, user2.email);

    let user3 = User {
        email: String::from("test3@example.com"),
        ..user2 // this takes the remaining fields from user1 and adds them to user3
    };

    // user2's fields have been moved into user3 and user2 cannot be used anymore. this is because there are strings left in user2 that are moved. if we added a new string username, it would copy.

    let user4 = User {
        email: String::from("test3@example.com"),
        username: String::from("test4"),
        ..user3 // this takes the remaining fields from user1 and adds them to user3
    };

    println!("user3 is still in scope because we created brand new string values for this user and thus it just had to move fixed value variables. {}", user3.username);

    let grey = Colour(27,26,25);

    println!("the blue value of grey is {}", grey.2);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // just like in javascript this is the same as email: email. this is called field init shorthand
        username, // and this is the same as username: username 
        active: true,
        sign_in_count: 1
    }
}
