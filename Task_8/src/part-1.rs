use std::fs;

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let mut lines: Vec<&str> = input.lines().collect();
    println!("Hello, world!");
}
