use strum_macros::EnumIter;

use self::Suit::*;
use self::Value::*;

use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

impl Suit {
    pub fn name(&self) -> String {
        match *self {
            Clubs => "Clubs".to_string(),
            Diamonds => "Diamonds".to_string(),
            Hearts => "Hearts".to_string(),
            Spades => "Spades".to_string()
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl Value {
    fn name(&self) -> String {
        match *self {
            Ace => "Ace".to_string(),
            Two => "Two".to_string(),
            Three => "Three".to_string(),
            Four => "Four".to_string(),
            Five => "Five".to_string(),
            Six => "Six".to_string(),
            Seven => "Seven".to_string(),
            Eight => "Eight".to_string(),
            Nine => "Nine".to_string(),
            Ten => "Ten".to_string(),
            Jack => "Jack".to_string(),
            Queen => "Queen".to_string(),
            King => "King".to_string(),
        }
    }

    pub fn value(&self) -> u32 {
        match *self {
            Ace => 14,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card { suit: suit, value: value }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.value().cmp(&other.value.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value.name(), self.suit.name())
    }
}
