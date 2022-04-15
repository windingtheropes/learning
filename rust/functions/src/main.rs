fn main() {
    println!("Hello, world!");
    another_function(5, 'c')
}

fn plus_one(x: u32) -> u32 {
    x + 1 // hi
}

fn another_function(x: i32, c: char) {
    println!("Another function! Paramater x is equal to {}, c is {}. Five plus one is {}.", x, c, plus_one(5))
}
