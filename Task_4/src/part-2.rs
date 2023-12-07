use std::fs::*;

struct Card {
    id: i32,
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

    pub fn get_clone(&self) -> Card {
        return Card {
            id: self.id.clone(),
            winning_numbers: self.winning_numbers.clone(),
            guessed_numbers: self.guessed_numbers.clone(),
        }
    }
}

fn main() {
    let input = read_to_string("./res/input2.txt").unwrap();
    let lines: Vec<&str> = input.split("\n").collect();

    let mut cards = Vec::<Card>::new();
    let mut counter = 1;
    for line in lines {
        cards.push(create_card(&line, counter));
        counter += 1;
    }

    let count_of_cards = recurse_cards(&cards, &cards,  counter);

    println!("{}", count_of_cards);
}

fn recurse_cards(org_cards: &Vec::<Card>, cards: &Vec::<Card>, max_card_id: i32) -> i32 {
    let mut new_cardset = Vec::<Card>::new();
    
    let mut i = 0;
    while i < cards.len() {
        let right_guesses = cards.get(i).unwrap().right_guesses();
        for j in 1..right_guesses+1 {
            let card = cards.get(i).unwrap();
            let id = card.id + j;
            if id <= max_card_id {
                new_cardset.push(org_cards.get((id-1) as usize).unwrap().get_clone());
            }
        }
        i += 1;
    }

    if !cards.is_empty() {
        return i as i32 + recurse_cards(org_cards, &new_cardset, max_card_id);
    } else {
        return 0;
    }
}

fn create_card(line: &str, i: i32) -> Card {
    let winning_guessed = line.split(": ").collect::<Vec<&str>>().get(1).unwrap().split(" | ").collect::<Vec<&str>>();
    let winning = winning_guessed.get(0).unwrap();
    let guessed = winning_guessed.get(1).unwrap();

    return Card {
        id: i,
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
