fn main() {
    let mut number = 3;

    let mut counter = 1;
    let res = loop {
        counter += 1;
        break counter;
    };

    let hello  = if counter == 2 { "hello" } else { "nothing" };
    println!("{}", hello);
    println!("{}", res);

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("start!");

    // inefficient for loop

    let a = [10, 20, 30];

    let mut index = 0;

    while index < 3 {
        println!("At index {} is {}", index, a[index]);

        index += 1;
    }
    
    // good for loop

    let mut index = 0;
    for item in a  { // automatically loops
        println!("at {} is {}", index, item);
        index += 1; // manually adding the index
    }

    // ranges and for instead of while 
    // 1..4 = 1, 2, 3
    // 1..=3 = 1, 2, 3 

    // reversed: 3, 2, 1
    for number in (1..=10).rev() {
        println!("{}", number);
    }
    println!("go")

}
