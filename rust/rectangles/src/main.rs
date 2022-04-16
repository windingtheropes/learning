// struct for method 3
#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle { // this is an associated function, it can be run by Rectangle::square, it does not have self as its first variable. used for creating new structs like String::from.
        Rectangle { width: size, height: size }
    }
    fn area(&self) -> u32 { // this is a method, it has self as the first variable, which is the struct data of which youre running the method from. 
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0 // r.width() returns true if the width is greater than 0, r.width is the actual wdith
    }
    fn height(&self) -> bool {
        self.height > 0 
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
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

    // fourth method, using struct methods
    
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("m4 The area of the rectangle is {} square pixels.", rect1.area()); // we use a reference because we do not want to give _3area's scope ownership of rectangle

    let rect2 = Rectangle { width:50, height:60 };
    let rect3 = Rectangle { width:20, height:40 };

    println!("{:?} can fit inside {:?}? {}", rect2, rect1, rect1.can_hold(&rect2));
    println!("{:?} can fit inside {:?}? {}", rect3, rect1, rect1.can_hold(&rect3));

    let square = dbg!(Rectangle::square(10));


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