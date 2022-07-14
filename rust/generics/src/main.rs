// the great news about generics is that they don't have a performance penalty in comparison with concrete types!
// using generics in Structs
struct Point<T> { // this struct can have values of any type and any value, so long as x and y are of the same type
    x: T,
    y: T,
}

struct Point2<T, U> { // using multiple type parameters, this point can have differing x and y values of any type.
    x: T,
    y: U,
}

// generics in enums
// generics are used for the option enum, allowing option to pass Some with any value of any type. versatile and simple.
enum Option<T> {
    Some(T),
    None,
}

// in the same way you can perform operations using result, which will either return the desired value of type T, or the error type corresponding to the operation, E. example could be reading a file, where T will be std::fs::File, and E would be std::io::Error
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// implementing logic for structs with generics can be done like below
// <T> must be specified after impl to indicate Point is a generic type and not concrete
impl<T> Point<T> {
    fn x(&self) -> &T { // return a reference so we aren't giving away ownership of anything
        &self.x
    }
}

// less abstract, you can implement constraints/logic for specific types of a generic struct
// below will be accessible if T in the Point is f32, and hence Point contains f32 values.
impl Point<f32> {
    // these mathmatical operations are only available for float types, so it would only make sense to constrain them to Point only if it contains f32 values.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2).sqrt())
    }
}

// generics within implementation of a generic struct!
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    // other is another Point, with possibly different x and y types, so X2 and Y2 are smart to include. This function will return a Point with x of type X1 and hance value of the first Point, and y of type Y2 hence of the 'other' parameter. 
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
// the point of this is to show that you can equally use generics in impl (such as impl<T>) and within methods within impl (such as mixup<X2, Y2>). 


fn main() {
    // because Point3 struct has X1 and Y1 generics, x and y can be different types from eachother, and any type for that matter
    // this is shown below
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    // Point3.mixup will take x from self, and y from other, to create a new Point3
    let p3 = p1.mixup(p2); // {x: 5, y: c}
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Notice we don't have to pass the type parameter, because it can be inferred by the values, since their types are equal to T and they can't be different
    let integer = Point { x: 5, y: 10 }; // integer will not have access to distance_from_origin
    let float = Point { x: 1.0, y: 4.0 }; // float will have access to distance_from_origin

    // This will not work
    // let wont_work = Point { x: 5, y: 4.0 };
    
    // This will work because x and y don't both have to follow the type T

    let will_work = Point2 {x: 5, y: 10.0 };

    // when defining generics they go in the signature of the function, the signature being FunctionName.
    // generics make code more flexible

    // below shows two functions that find the largest value in a slice, one for characters and one for i32 values:

    // finds largest i32 value
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    // finds largest character
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list); // the largest number value in the list above

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list); // the largest character by its number value in the list above

    // the only difference between these two functions largest_i32 and largest_char is the parameter types, as literally the rest of the code is duplicate
    // we will introduce a generic type parameter in a single function to solve this
    // you can parameterize the types in a function, just as value paramaters need names, so do type parameters
    // you can use any identifier, but T is common and follows rust's short naming conventions

    // fn largest<T>(list: &[T]) -> T { // T can be any type, at this state, taking T as input, and returning T as output, where T is the type of value, and the value is determined by list's values as normal
    //     // now we can add the same code from either one of the largest functions
    //     let mut largest = list[0];

    //     for &item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }
    // this code will not compile, though, because you can pass any type to it, and not all types can be iterated, nor their values compared mathmatically



}
