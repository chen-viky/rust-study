use std::io;

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(" Failed to read line");
    println!("You raw input is: {:?}.", input);
    let number: i64 = input.trim().parse().expect("Please type a number!");
    println!("Your input is: {:?}.", number);
}
