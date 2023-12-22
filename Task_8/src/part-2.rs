use std::fs;
use std::collections::HashMap;

const INPUT_PATH: &str = "./res/input2.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let lines: Vec<&str> = input.lines().collect();

    let instructions = *lines.get(0).unwrap();
    let mut instruction_cursor = 0usize;

    let mut nodes = HashMap::<&str, (&str, &str)>::new();

    lines.iter().skip(1).for_each(|line| {
        let mut line = line.chars();
        line.next_back();
        let line = line.as_str();

        let line_list = line.split(" = (").collect::<Vec<&str>>();
        let node_name = line_list.get(0).unwrap();

        let line_list = line_list.get(1).unwrap().split(", ").collect::<Vec<&str>>();
        let left = line_list.get(0).unwrap();
        let right = line_list.get(1).unwrap();

        nodes.insert(*node_name, (*left, *right));
    });

    let mut next_node_ids = nodes.keys().into_iter().filter(|id| id.chars().nth(2).unwrap() == 'A').collect::<Vec<&&str>>();
    let mut cycle_lengths = vec![0u64;6];

    let mut go_on = true;
    while !next_node_ids.iter().all(|node_id| node_id.chars().nth(2).unwrap() == 'Z') && go_on {
        let instruction = instructions.chars().nth(instruction_cursor % instructions.len()).unwrap();
        
        for i in 0..next_node_ids.len() {
            let node_id = **next_node_ids.get(i).unwrap();

            if next_node_ids.get(i).unwrap().chars().nth(2).unwrap() == 'Z' {
                match cycle_lengths.get(i).unwrap() {
                    0 => {
                        cycle_lengths[i] = instruction_cursor as u64;
                    },
                    _ => {
                        ()
                    },
                        
                }
            }

            if cycle_lengths.iter().all(|n| *n > 0) {
                go_on = false;
            }

            match instruction {
                'L' => {
                    next_node_ids.remove(i);
                    next_node_ids.insert(i, &nodes.get(node_id).unwrap().0);
                },
                'R' => {
                    next_node_ids.remove(i);
                    next_node_ids.insert(i, &nodes.get(node_id).unwrap().1);
                },
                _ => panic!("what the heeeeeeelll, ohhhhh my gaa-aa-awd"),
            }
        }

        instruction_cursor += 1;
    }
    
    println!("{:?}", gcd_of(&cycle_lengths));
    // println!("{:?}", instruction_cursor);
}
fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn gcd_of(list: &Vec<u64>) -> u64 {
    let mut iter = list.iter();
    let first = *iter.next().unwrap();
    let second = *iter.next().unwrap();

    let mut ans = gcd(first, second);
    while let Some(&next) = iter.next() {
        ans = gcd(ans, next);
    }
    ans
}