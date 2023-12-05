use std::fs::*;
use std::string::*;

fn main() {
    let strings_literal_spelled = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
        ];
    

    let input_filename = "./res/input2.txt";
    let input = read_to_string(input_filename).unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let mut first_occurence_indices: [i32; 18] = [0; 18];
        let mut last_occurence_indices: [i32; 18] = [0; 18];

        for i in 0..first_occurence_indices.len() {
            let f = line.find(strings_literal_spelled[i]);
            first_occurence_indices[i] = match f {
                Some(val) => val as i32,
                None => i32::MAX,
            } ;
            let rf = line.rfind(strings_literal_spelled[i]);
            last_occurence_indices[i] = match rf {
                Some(val) => val as i32,
                None => i32::MIN,
            } ;
        }

        let idx_e_first = get_index_and_smallest_element(first_occurence_indices);
        let idx_e_latest = get_index_and_biggest_element(last_occurence_indices);

        let idx_for_val_first = idx_e_first % 9;
        let idx_for_val_last = idx_e_latest % 9;

        let mut first_number_in_line = strings_literal_spelled[idx_for_val_first as usize].to_string();
        let last_number_in_line = strings_literal_spelled[idx_for_val_last as usize];

        first_number_in_line.push_str(last_number_in_line);
        let calibration_val = first_number_in_line.parse::<u32>().unwrap();

        sum += calibration_val

    }

    println!("{}", sum);
}

fn get_index_and_smallest_element(arr: [i32; 18]) -> i32 {
    let mut index = 0;
    let mut smallest_element = i32::MAX;
    for i in 0..arr.len() {
        if arr[i] < smallest_element {
            smallest_element = arr[i];
            index = i;
        }
    }
    return index as i32;
}

fn get_index_and_biggest_element(arr: [i32; 18]) -> i32 {
    let mut index = 0;
    let mut biggest_element = 0;
    for i in 0..arr.len() {
        if arr[i] > biggest_element {
            biggest_element = arr[i];
            index = i;
        }
    }
    return index as i32;
}