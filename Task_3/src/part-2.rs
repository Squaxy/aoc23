use std::{fs::*, collections::HashMap};

const DIGITS: &str = "0123456789";
const GEAR_SYMBOL: &str = "*";

#[derive(PartialEq, Eq, Hash)]
struct Gear {
    number: String,
    x: usize,
    y: usize,
    len: usize,
}

impl Gear {
    fn get_clone(&self) -> Gear {
        return Gear {
            number: self.number.clone(),
            x: self.x.clone(),
            y: self.y.clone(),
            len: self.len.clone(),
        };
    }
}

fn main() {
    let input = read_to_string("./res/input2.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();
    let mut gear_numbers = Vec::<Gear>::new();
    let mut symbol_indices = Vec::<[usize; 2]>::new();

    let max_index_x: usize = lines.len()-1;
    let max_index_y: usize = lines.get(0).unwrap().len()-1;

    iterate_lines(lines, &mut gear_numbers, &mut symbol_indices);

    let actual_gear_values = Vec::<i32>::new();

    let sum = iterate_gears(&gear_numbers, &symbol_indices, &max_index_x, &max_index_y);

    println!("all n cnt: {}", gear_numbers.len());
    println!("gear val cnt: {}", actual_gear_values.len());

    println!("{}", sum);

}

fn iterate_gears(gear_numbers: &Vec<Gear>, symbol_indices: &Vec<[usize; 2]>, max_index_x: &usize, max_index_y: &usize) -> i32 {
    
    let mut gear_proximity_map = HashMap::<Gear, Vec<[usize; 2]>>::new(); 

    for gear in gear_numbers {
        let x = gear.x as i32;
        let y = gear.y as i32;
        
        let proximity: Vec<[usize; 2]> = (x - 1..=x + 1)
            .flat_map(|i| (y - 1..=y + gear.len as i32).map(move |j| [i as usize, j as usize]))
            .filter(|&neighbor| neighbor[0] < *max_index_x && neighbor[1] < *max_index_y)
            .collect();

        if proximity.iter().any(|&neighbor| symbol_indices.contains(&neighbor)) {
            // actual_gear_values.push(gear.number.parse().unwrap());
            gear_proximity_map.insert(gear.get_clone(), proximity);
        }
    }

    let mut gear_sum = 0;
    
    for sym_i in symbol_indices {
        let mut gears_in_proximity_to_symbol = Vec::<Gear>::new();
        let mut gear_count = 0;
        for gear_proxy_tuple in &gear_proximity_map {
            if gear_proxy_tuple.1.contains(sym_i) {
                gear_count += 1;
                gears_in_proximity_to_symbol.push(gear_proxy_tuple.0.get_clone());
            }
        }
        if gear_count == 2 {
            gear_sum += gear_ratio(&gears_in_proximity_to_symbol.get(0).unwrap(), &gears_in_proximity_to_symbol.get(1).unwrap());
        }
    }
    return gear_sum;
}

fn gear_ratio(g1: &Gear, g2: &Gear) -> i32 {
    return g1.number.parse::<i32>().unwrap() * g2.number.parse::<i32>().unwrap();
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
    return GEAR_SYMBOL.contains(*c);
}
