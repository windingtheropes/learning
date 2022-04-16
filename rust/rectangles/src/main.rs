// struct for method 3
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

fn main() {

    // first method, using single variables, and _1area()
    // issue: technically there's no connection between width1 and height1

    let width1 = 30;
    let height1 = 50;

    println!("m1 The area of the rectangle is {} square pixels.", _1area(width1, height1));

    // second method, using tuples, and _2area()
    // issue: tuples dont have named elements so we don't technically know what is height and what is width

    let rect1 = (30, 50);

    println!("m2 The area of the rectangle is {} square pixels.", _2area(rect1));

    // third method, using structs, and _3area()
    // the struct deffinition is at the top

    let rect1 = Rectangle { 
        width: dbg!(30 * 2), // this will print the result of the expression, plus still providing the value of 60 to width of the rectangle
        height: 50, };

    println!("m3 The area of the rectangle is {} square pixels.", _3area(&rect1)); // we use a reference because we do not want to give _3area's scope ownership of rectangle

    println!("rect1 is {:?}", rect1); // :? specifies that we want to print rect1 in output format 'debug'
    println!("rect1 is {:#?}", rect1); // pretty print

    dbg!(&rect1); // dbg takes ownership so must pass a reference. it also prints to stderror. dbg also returns the value of the expression. 
}

fn _1area(width: u32, height: u32) -> u32 {
    width * height
}

fn _2area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn _3area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}