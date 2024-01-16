pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    // let's do some formatting in rust
    println!("Number : {}", 1);

    // if we want to have multiple placeholders
    println!("{} is from {}", "Dhruva", "Mass");

    // positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Dhruva", "Mass", "code"
    );

    // named Arguments

    println!(
        "{name} likes to play {activity}",
        name = "Dhruva",
        activity = "Coding"
    );

    // place holder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "dhruva"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
