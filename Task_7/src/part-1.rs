mod card;

use card::Card;

use std::{fs, str::FromStr};

const INPUT_PATH: &str = "./res/input.txt";

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // hier kommt der elbow grease
        // verweise auf card cmp im falle das zwei hände sonst gleich sind
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Hands = Vec<Hand>;

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let lines: Vec<&str> = input.lines().collect();
    
    let mut hands: Hands = lines.iter().filter_map(|line| {
        let segments: Vec<&str> = line.split_whitespace().collect();

        let cards: Vec<Card> = segments.get(0)?
        .chars()
        .filter_map(|c| Card::from_str(&c.to_string()).ok())
        .collect();

        let bid: u64 = segments.get(1)?.parse().ok()?;

        Some(Hand {cards, bid})
    }).collect();

    // hands.sort();

    println!("{:?}", accumulate_sorted_hands_ranks(&hands));
}

fn accumulate_sorted_hands_ranks(hands: &Hands) -> u128 {
    hands.iter()
        .enumerate()
        .map(|(index, hand)| hand.bid as u128 * (index as u128 + 1))
        .sum()
}
