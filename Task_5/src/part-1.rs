use std::fs::*;

fn main() {
    let input = read_to_string("./res/input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    println!("Hello, world!");
}
