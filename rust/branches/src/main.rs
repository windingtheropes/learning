fn main() {
    let condition = false;
    let number = if condition {1} else {111};
    if number < 10 {
        println!("Number is less than 10")
    }
    else
    {
        println!("Number is not less than 10")
    }
}
