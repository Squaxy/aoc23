use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Card {
    J = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    T = 10,
    Q = 11,
    K = 12,
    A = 13, 
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(Card::J),
            "2" => Ok(Card::TWO),
            "3" => Ok(Card::THREE),
            "4" => Ok(Card::FOUR),
            "5" => Ok(Card::FIVE),
            "6" => Ok(Card::SIX),
            "7" => Ok(Card::SEVEN),
            "8" => Ok(Card::EIGHT),
            "9" => Ok(Card::NINE),
            "T" => Ok(Card::T),
            "Q" => Ok(Card::Q),
            "K" => Ok(Card::K),
            "A" => Ok(Card::A),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_representation = match self {
            Card::J => "J",
            Card::TWO => "2",
            Card::THREE => "3",
            Card::FOUR => "4",
            Card::FIVE => "5",
            Card::SIX => "6",
            Card::SEVEN => "7",
            Card::EIGHT => "8",
            Card::NINE => "9",
            Card::T => "T",
            Card::Q => "Q",
            Card::K => "K",
            Card::A => "A",
        };
        write!(f, "{}", str_representation)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}