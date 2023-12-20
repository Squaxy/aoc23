mod card;

use card::Card;

use std::{fs, str::FromStr};

const INPUT_PATH: &str = "./res/input.txt";

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
    is: u8,
}

impl Hand {
    fn set_is(&mut self) -> () {
        if self.is_five_of_a_kind() {
            self.is = 6;
        } else if self.is_four_of_a_kind() {
            self.is = 5;
        } else if self.is_fullhouse() {
            self.is = 4;
        } else if self.is_three_of_a_kind() {
            self.is = 3;
        } else if self.is_two_pair() {
            self.is = 2;
        } else if self.is_one_pair() {
            self.is = 1;
        }
    }
    fn is_five_of_a_kind(&self) -> bool {
        for i in 0..(self.cards.len()-1) {
            if self.cards.get(i).unwrap() != self.cards.get(self.cards.len()-1).unwrap() {
                return false;
            }
        }
        return true;
    }
    fn is_four_of_a_kind(&self) -> bool {
        let mut card_type_array = [0u8; 15];
        for card in &self.cards {
            card_type_array[*card as usize] += 1;
        }
        return card_type_array.iter().any(|v| *v == 4);
    }
    fn is_fullhouse(&self) -> bool {
        let mut card_type_array = [0u8; 15];
        for card in &self.cards {
            card_type_array[*card as usize] += 1;
        }
        return card_type_array.iter().any(|v| *v == 3) && card_type_array.iter().any(|v| *v == 2);
    }
    fn is_three_of_a_kind(&self) -> bool {
        let mut card_type_array = [0u8; 15];
        for card in &self.cards {
            card_type_array[*card as usize] += 1;
        }
        return card_type_array.iter().any(|v| *v == 3);
    }
    fn is_two_pair(&self) -> bool {
        let mut card_type_array = [0u8; 15];
        for card in &self.cards {
            card_type_array[*card as usize] += 1;
        }
        return card_type_array.iter().filter(|v| **v == 2).collect::<Vec<&u8>>().len() == 2;
    }
    fn is_one_pair(&self) -> bool {
        let mut card_type_array = [0u8; 15];
        for card in &self.cards {
            card_type_array[*card as usize] += 1;
        }
        return card_type_array.iter().any(|v| *v == 2);
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is > other.is {
            return std::cmp::Ordering::Greater;
        } else if self.is < other.is {
            return std::cmp::Ordering::Less;
        } else {
            return compare_single_cards(self, other);
        }
    }
}
fn compare_single_cards(hand: &Hand, other: &Hand) -> std::cmp::Ordering {
    for i in 0..hand.cards.len() {
        if hand.cards.get(i).unwrap() > other.cards.get(i).unwrap() {
            return std::cmp::Ordering::Greater;
        } else if hand.cards.get(i).unwrap() < other.cards.get(i).unwrap() {
            return std::cmp::Ordering::Less;
        }
    }
    return std::cmp::Ordering::Equal;
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type Hands = Vec<Hand>;

fn main() {
    let input: String = fs::read_to_string(INPUT_PATH).expect("Failed to read file!");
    let mut lines: Vec<&str> = input.lines().collect();
    
    let mut hands: Hands = lines.iter_mut().filter_map(|line| {
        let segments: Vec<&str> = line.split_whitespace().collect();

        let cards: Vec<Card> = segments.get(0)?
        .chars()
        .filter_map(|c| Card::from_str(&c.to_string()).ok())
        .collect();

        let bid: u64 = segments.get(1)?.parse().ok()?;
        let is = 0u8;
        
        Some(Hand {cards, bid, is})
    }).collect();

    hands.iter_mut().for_each(|hand| hand.set_is());
    hands.sort();

    println!("{:?}", accumulate_sorted_hands_ranks(&hands));
}

fn accumulate_sorted_hands_ranks(hands: &Hands) -> u128 {
    hands.iter()
        .enumerate()
        .map(|(index, hand)| hand.bid as u128 * (index as u128 + 1))
        .sum()
}
