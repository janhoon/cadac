use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_int: u32 = input
        .trim()
        .parse()
        .expect("Failed to parse input as integer");

    println!("Input: {input_int}");

    println!("Hello, world!");
}
