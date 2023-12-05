use std::fs::*;
use std::string::*;

fn main() {
    let input_filename = "./res/input.txt";
    let input = read_to_string(input_filename).unwrap();

    let output = input.lines().map(|line| {
        let mut it = line.chars().filter_map(|c| {
            c.to_digit(10)
        });

        let first = it.next().expect("should be a number");
        let last = it.last();

        match last {
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }
        .parse::<u32>()
        .expect("should be a number")
    })
    .sum::<u32>();

    println!("{}", output.to_string());
}
