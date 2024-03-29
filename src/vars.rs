// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // let keyword is used similar to js
    let name = "Dhruva";
    let mut age = 23;

    age = 24;

    println!("My name is {} and I am {}", name, age);

    // there is also const keyword
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign mutliple Variables

    let (my_name, my_age) = ("Dhruva", 23);
    println!("{} is {}", my_name, my_age);
}
