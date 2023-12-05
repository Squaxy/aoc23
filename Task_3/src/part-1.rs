use std::fs::*;

const NON_SYMBOLS: &str = "0123456789.";
const DIGITS: &str = "0123456789";

struct Gear {
    number: String,
    x: usize,
    y: usize,
    len: usize,
}

fn main() {
    let input = read_to_string("./res/input.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let mut gear_numbers = Vec::<Gear>::new();
    let mut symbol_indices = Vec::<[usize; 2]>::new();

    let max_index_x: usize = lines.len()-1;
    let max_index_y: usize = lines.get(0).unwrap().len()-1;

    iterate_lines(lines, &mut gear_numbers, &mut symbol_indices);

    let mut actual_gear_values = Vec::<i32>::new();

    iterate_gears(&gear_numbers, &symbol_indices, &mut actual_gear_values, &max_index_x, &max_index_y);

    // for el in &actual_gear_values {
    //     println!("{}", el)
    // }
    println!("all n cnt: {}", gear_numbers.len());
    println!("gear val cnt: {}", actual_gear_values.len());

    println!("{}", actual_gear_values.iter().sum::<i32>());

}

fn iterate_gears(gear_numbers: &Vec<Gear>, symbol_indices: &Vec<[usize; 2]>, actual_gear_values: &mut Vec<i32>,
    max_index_x: &usize, max_index_y: &usize) -> () {
    
    for gear in gear_numbers {
        let mut proximity = Vec::<[usize; 2]>::new();
        let x = gear.x as i32;
        let y = gear.y as i32;
        for i in x-1..(x+2) {
            if i < 0 || i > *max_index_x as i32 {
                continue;
            }
            for j in y-1..(y+gear.len as i32+1) {
                if j < 0 || j > *max_index_y as i32{
                    continue;
                }
                proximity.push([i as usize, j as usize]);
            }
        }
        for neighbor in proximity {
            if symbol_indices.contains(&neighbor) {
                actual_gear_values.push(gear.number.parse::<i32>().unwrap());
                break;
            }
        }
    }
}

fn iterate_lines(lines: Vec<&str>, gear_numbers: &mut Vec<Gear>, symbol_indices: &mut Vec<[usize; 2]>) -> () {
    for i in 0..lines.len() {
        let line = lines.get(i).unwrap();
        let mut number = String::new();
        let mut number_first_digit_index: usize = 0;

        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();

            if DIGITS.contains(c) {
                if number.is_empty() {
                    number_first_digit_index = j;
                }
                number.push(c);
                continue;
            }

            if is_symbol(&c) {
                symbol_indices.push([i, j]);
            }

            if !number.is_empty() {
                gear_numbers.push(Gear {
                    number: number.clone(),
                    x: i,
                    y: number_first_digit_index,
                    len: number.len()
                });
                number = String::new();
            }

        }
    }
}

fn is_symbol(c: &char) -> bool {
    return !NON_SYMBOLS.contains(*c);
}
