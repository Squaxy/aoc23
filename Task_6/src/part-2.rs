use std::fs;

const INPUT_PATH: &str = "./res/input.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let lines: Vec<&str> = input.split("\n").collect();
    let times: Vec<u64> = extract_numbers(lines.get(0).expect("first line read wrong"));
    let distances: Vec<u64> = extract_numbers(lines.get(1).expect("second line read wrong"));

    let result: Option<u64> = times.iter()
    .zip(distances.iter())
    .map(|tuple| count_of_ways_to_beat(tuple.0, tuple.1))
    .reduce(|a, b| a*b);

    println!("{}", result.expect("something went wrong in the pipeline!"));
}

fn count_of_ways_to_beat(time: &u64, distance: &u64) -> u64 {
    let mut counter_of_wins: u64 = 0;
    for pressed_duration in 0..*time {
        let this_distance = (time - pressed_duration) * pressed_duration;
        if this_distance > *distance {
            counter_of_wins += 1;
        }
    }
    return counter_of_wins;
}

fn extract_numbers(input: &str) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    let mut current_number: u64 = 0;
    let sanitized_input = input.replace(" ", "");
    for c in sanitized_input.chars() {
        if let Some(digit) = c.to_digit(10) {
            current_number = current_number * 10 + digit as u64;
        } else if current_number > 0 {
            result.push(current_number);
            current_number = 0;
        }
    }

    if current_number > 0 {
        result.push(current_number);
    }

    result
}