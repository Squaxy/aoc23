use std::fs::*;

struct Card {
    winning_numbers: Vec<i32>,
    guessed_numbers: Vec<i32>,
}

impl Card {
    pub fn right_guesses(&self) -> i32 {
        let mut right_guesses: i32 = 0;
        for n in &self.guessed_numbers {
            if self.winning_numbers.contains(&n) {
                right_guesses += 1;
            }
        }
        return right_guesses;
    }
}

fn main() {
    let input = read_to_string("./res/input2.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut cards = Vec::<Card>::new();
    for line in lines {
        cards.push(create_card(&line));
    }

    // recursive method for cards altering given reference?
    // google recursion for rust

    println!("{}", sum);
}

fn create_card(line: &str) -> Card {
    let winning_guessed = line.split(": ").collect::<Vec<&str>>().get(1).unwrap().split(" | ").collect::<Vec<&str>>();
    let winning = winning_guessed.get(0).unwrap();
    let guessed = winning_guessed.get(1).unwrap();

    return Card {
        winning_numbers: string_to_vec(winning),
        guessed_numbers: string_to_vec(guessed),
    };   
}

fn string_to_vec(s: &str) -> Vec<i32> {
    let mut numbers = Vec::<i32>::new();

    let mut i = 0;
    while i < s.len() {
        let mut n = String::new();
        let c1 = s.chars().nth(i).unwrap();
        let c2 = s.chars().nth(i+1).unwrap();
        if c1.is_ascii_digit() {
            n.push(c1)
        }
        if c2.is_ascii_digit() {
            n.push(c2)
        }
        if !n.is_empty() {
            numbers.push(n.parse::<i32>().unwrap());
        }
        i += 3;
    }

    return numbers;
}
