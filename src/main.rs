use std::io;

fn main() {
    println!("Hello, world!");

    println!("Enter a number:");

    let mut num_str = String::new();

    io::stdin()
        .read_line(&mut num_str)
        .expect("Failed to read line!");

    let num: i32 = num_str.trim().parse().expect("Not a number!");

    println!("You Entered: {}", num);
}
