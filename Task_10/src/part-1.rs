mod symbol;
mod maze;

use symbol::Symbol;
use maze::Maze;

use std::fs;

const INPUT_PATH: &str = "./res/input.txt";

const _TEST_INPUT: &str = "1 2 3 4 5 6 7 8";
const _TEST_INPUT_COMPLEX: &str = "\
1 2 3 4 5 6 7 8\n\
11 12 13 14 15\
";

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let lines: Vec<&str> = input.lines().collect();

    println!("Sum of the interpolated values is -> {}", 6);
}
