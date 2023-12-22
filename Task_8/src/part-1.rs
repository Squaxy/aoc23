use std::fs;
use std::collections::HashMap;

const INPUT_PATH: &str = "./res/input.txt";

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

    let mut next_node_id = "AAA";

    while next_node_id != "ZZZ" {
        let instruction = instructions.chars().nth(instruction_cursor % instructions.len()).unwrap();
        next_node_id = match instruction {
            'L' => nodes.get(next_node_id).unwrap().0,
            'R' => nodes.get(next_node_id).unwrap().1,
            _ => panic!("this cant be")
        };
        instruction_cursor += 1;
    }

    println!("{:?}", instruction_cursor);
}
