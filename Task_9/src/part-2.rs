use std::fs;

const INPUT_PATH: &str = "./res/input.csv";

const _TEST_INPUT: &str = "1 2 3 4 5 6 7 8";
const _TEST_INPUT_COMPLEX: &str = "\
1 2 3 4 5 6 7 8\n\
11 12 13 14 15\
";

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let lines: Vec<&str> = input.lines().collect();
    let array_of_numbers: Vec<Vec<i32>> =format_input_lines(&lines);

    let array_of_next_values: Vec<i32> = extrapolate_values(&array_of_numbers);
    let sum: i32 = array_of_next_values.iter().sum();

    println!("Sum of the interpolated values is -> {sum}");
}

fn format_input_lines(lines: &Vec<&str>) -> Vec<Vec<i32>> {
    lines.iter().map(|line| {
        line.split_ascii_whitespace()
        .into_iter()
        .map(|string_number| string_number.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>()
}

fn extrapolate_values(array_of_numbers: &Vec<Vec<i32>>) -> Vec<i32> {
    array_of_numbers.iter().map(|numbers| extrapolate_value(numbers)).collect()
}

fn extrapolate_value(numbers: &Vec<i32>) -> i32 {
    let mut array_of_numbers: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    array_of_numbers.push(numbers.clone());
    *extrapolate_value_recurse(&mut array_of_numbers).first().unwrap().first().unwrap()
}

fn extrapolate_value_recurse(array_of_numbers: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    if array_of_numbers.last().unwrap().iter().all(|n| *n == 0) {
        return extrapolate_once(&mut array_of_numbers.clone());
    }

    array_of_numbers.push(calculate_diff_sequence(&array_of_numbers.last().expect("empty vec called this")));
    extrapolate_value_recurse(array_of_numbers)
}

fn calculate_diff_sequence(sequence: &Vec<i32>) -> Vec<i32> {
    let mut diff_sequence = Vec::<i32>::new();
    for i in 0..sequence.len()-1 {
        diff_sequence.push(sequence[i+1] - sequence[i] )
    }
    diff_sequence
}

fn extrapolate_once(array_of_numbers: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    for i in 1..array_of_numbers.len() {
        let clone = array_of_numbers.clone();
        let shallow_i = array_of_numbers.len()-1-i.clone();
        let shallow = clone.get(shallow_i).unwrap().first().unwrap();
        let deeper = clone.get(array_of_numbers.len()-i).unwrap().first().unwrap();
        array_of_numbers.get_mut(shallow_i).unwrap().insert(0, *shallow - *deeper);
    }

    println!("interpolated: {:?}", array_of_numbers.clone());
    array_of_numbers.clone()
}