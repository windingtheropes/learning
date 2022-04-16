struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1
    }
}
