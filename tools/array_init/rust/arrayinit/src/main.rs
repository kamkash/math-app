// src/main.rs

// Declare the library crate (it has the same name as your project)
extern crate arrayinit;

// Bring items from the library into scope
use arrayinit::add;
use arrayinit::utils::greet;

fn main() {
    let sum = add(5, 3);
    println!("The sum is: {}", sum);

    let greeting = greet("User");
    println!("{}", greeting);
}
