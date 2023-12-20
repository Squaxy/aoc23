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
        todo!();
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
    let lines: Vec<&str> = input.split("\n").collect();
    
    // example line: "557T5 626"
    let hands: Hands = lines.iter().map(|line| {
        let segments: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

        let cards: Vec<Card> = segments.get(0).expect("someone had no hand :O")
        .chars()
        .map(|c| Card::from_str(&c.to_string()).expect("failed to parse the hand"))
        .collect::<Vec<Card>>();

        let bid: u64 = segments.get(1)
        .expect("someone had no bid :O")
        .parse::<u64>()
        .expect("couldnt convert bid str to u64");

        Hand {
            cards: cards,
            bid: bid,
        }
    }).collect();

    println!("{:?}", hands);
}
