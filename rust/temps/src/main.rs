use std::io;

// K is kel
// F is far
// C is cel


fn main() {
    
    println!("Welcome to the temperature converter.");
    println!("Please enter the first value.");

    let mut v1 = String::new();

    io::stdin()
        .read_line(&mut v1)
        .expect("Please enter a valid integer.");

    let v1 : i32 = match v1.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a valid integer."),
    };

    println!("{}C is {}F", v1, c_to_f(v1));
    println!("{}F is {}C", v1, f_to_c(v1));

    println!("{}C is {}F", v1, c_to_k(v1));
    println!("{}F is {}C", v1, f_to_k(v1));

    println!("{}K is {}F", v1, k_to_f(v1));
    println!("{}K is {}C", v1, k_to_c(v1));

}

fn c_to_f(c: i32) -> f32{
    return (c as f32 * 9.0/5.0) + 32.0
}

fn f_to_c(f: i32) -> f32{
    return (f as f32 - 32.0) * 5.0/9.0
}

fn f_to_k(f: i32) -> f32{
    return (f as f32 - 32.0) * 5.0/9.0 + 273.15
}

fn c_to_k(c: i32) -> f32{
    return c as f32 + 273.15
}

fn k_to_c(k: i32) -> f32{
    return k as f32 - 273.15
}

fn k_to_f(k: i32) -> f32{
    return (k as f32 - 273.15) * 9.0/5.0 + 32.0
}
