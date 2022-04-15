fn main() {
    for instance in (1..=12) {
        let suffix = if instance == 1 { "st" } else if instance == 2 { "nd" } else if instance == 3 { "rd" } else { "th" };
        println!("On the {}{} day of christmas lyrics happened.", instance, suffix);
    }
}
