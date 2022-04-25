// siblings can access eachother, otherwise modules, functions, enums, structs need to be public to be accessed.

// hosting is also in front_of_house.rs, so we can import it like 
// pub use create::front_of_house::hosting

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve_order() {}

mod back_of_house {

    pub enum Appetizer { // the enum and all of its fields will be public since we added pub at the deffinition
        Soup,
        Salad,
    }
    // the customer picks the bread, the chef chooses the fruit based on the season

    pub struct Breakfast {
        pub toast: String, // fields can be defined as public or private on a case to case basis
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // since Breakfast has a private value this function must be public
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super is like using .. in a filesystem
        // in this normal scope the furthest back you could go is cook_order(), the same as doing . in a filesystem
        // super gets you to the module's parent, which in this case is crate.
    }

    fn cook_order() {}
}

// exact path
// use crate::front_of_house::hosting;

//relative path
use self::front_of_house::hosting;

// public import, this makes front_of_hpouse::hosting accessible to external code as well
pub use self::front_of_house::hosting;
// back of house thinks about foh, but typically foh doesnt think about boh

//using idiomatic paths is smarter so you don't get confused what scope the module is from,
// though we could do 
// use self::front_of_house::hosting::add_to_waitlist() it would be unclear where the parent of this function was from. this is why we import the parent, called iodiomatc paths.

pub fn eat_at_restaurant() {

    hosting::add_to_waitlist(); // now since we brought hosting into scope this is all we have to do to access front_of_house::hosting and its children

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast) ;

    // the customer *cannot* modify or see the seasonal fruit because it is private
    // meal.seasonal_fruit = String::from("blueberries");

    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path, foh is accessible because it's a sibling
    front_of_house::hosting::add_to_waitlist();

}

// idiomatic way of importing hashmap

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// IMPORTING A MODULE
// importing modules that have identical named children

use std::io;
use std::fmt;

// we can't import std::io::Result *and* std::fmt::Result because they have the same names

//both have a Result type,
// so we need to specify which of the identical types we are referring to
fn function1() -> fmt::Result {

}

fn function1() -> io::Result {

}

// IMPORTING WITH AS
// but, we can import modules as names with `as`

use std::fmt::Result; 
use std::io::Result as IoResult; // this imports Result from io under the name IoResult, so we can use Result from std::fmt

fn function1() -> Result {

}

fn function1() -> ioResult<()> {

}

// nested paths
use std::io::{Error, ErrorKind};

// instead of using two imports like this
// use std::io::Error;
// use std::io::ErrorKind;

// if we want to import std::io and std::io:Write inside the same nested path, we can use self, which will import the parent, like so
use std::io::{Self, Write};

// glob operator
// if we want to import all children of std::io, we can do this
// use std::io::*;