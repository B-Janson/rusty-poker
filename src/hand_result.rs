use crate::card::*;

use std::collections::HashMap;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub struct HandResult {
    result: u32,
    highest: Card
}

impl HandResult {
    pub fn check_hand(hand: &mut [Card]) -> HandResult {
        if hand.len() == 0 {
            panic!("Panicked due to checking an empty hand.");
        }

        let flush_check = Self::check_flush(hand);
        if flush_check.result == 8 {
            return flush_check;
        }

        let of_a_kind_check = Self::check_of_a_kind(hand);
        if of_a_kind_check.result == 7 {
            return of_a_kind_check;
        }

        if of_a_kind_check.result == 6 {
            return of_a_kind_check;
        }

        if flush_check.result == 5 {
            return flush_check;
        }

        let straight_check = Self::check_straight(hand);
        if straight_check.result == 4 {
            return straight_check;
        }

        if of_a_kind_check.result == 3 {
            return of_a_kind_check;
        }

        if of_a_kind_check.result == 2 {
            return of_a_kind_check;
        }

        if of_a_kind_check.result == 1 {
            return of_a_kind_check;
        }

        hand.sort_by(|a, b| a.value.value().cmp(&b.value.value()));

        return HandResult {
            result: 0,
            highest: *hand.last().unwrap()
        }
    }

    fn check_flush(hand: &mut [Card]) -> HandResult {
        let mut result = 0;
        let mut highest = hand[0];

        // Make a hashmap to keep track of the number of cards of the same suit.
        let mut card_map = HashMap::<Suit, u32>::new();

        // For each card
        for card in hand.iter() {
            // If this card's suit is already in the map, increment its count and
            // re-insert
            if let Some(val) = card_map.get(&card.suit) {
                let val = &(val + 1);
                card_map.insert(card.suit, *val);
            } else {
                card_map.insert(card.suit, 1);
            }
        }

        for (s, n) in card_map.into_iter() {
            if n >= 5 {
                let mut flush_cards = hand.iter().filter(|&c| c.suit == s).cloned().collect::<Vec<Card>>();

                let test = Self::check_straight(&mut flush_cards);

                if test.result == 4 {
                    result = 8;
                } else {
                    result = 5;
                }

                highest = *flush_cards.last().unwrap();
            }
        }

        HandResult {
            result: result,
            highest: highest
        }
    }

    fn check_of_a_kind(hand: &mut [Card]) -> HandResult {
        let mut result = 0;
        let mut highest = hand[0];

        // Make a hashmap to keep track of the number of cards of the same value.
        let mut card_map = HashMap::<Value, Vec<Card>>::new();

        // For each card
        for card in hand.iter() {
            // If this card's suit is already in the map, increment its count and
            // re-insert
            if let Some(val) = card_map.get(&card.value) {
                let mut a = val.clone();
                a.push(*card);
                card_map.insert(card.value, a.to_vec());
            } else {
                card_map.insert(card.value, vec![*card]);
            }
        }

        let mut final_cards = Vec::<Card>::new();

        for (v, n) in card_map.into_iter() {
            // println!("v: {:?} n: {:#?}", v, n);

            if n.len() == 4 {
                return HandResult {
                    result: 7,
                    highest: hand.iter().filter(|&c| c.value == v).cloned().collect::<Vec<Card>>()[0]
                }
            }

            if n.len() > 1 {
                final_cards.extend(n);
            }
        }

        final_cards.sort_by(|a, b| a.value.value().cmp(&b.value.value()));

        if final_cards.len() == 5 || final_cards.len() == 7 {
            result = 6;
            highest = *final_cards.last().unwrap();
        } else if final_cards.len() == 4 || final_cards.len() == 6 {
            result = 2;
            highest = *final_cards.last().unwrap();
        } else if final_cards.len() == 3 {
            result = 3;
            highest = *final_cards.last().unwrap();
        } else if final_cards.len() == 2 {
            result = 1;
            highest = *final_cards.last().unwrap();
        }

        HandResult {
            result: result,
            highest: highest
        }
    }

    fn check_straight(hand: &mut [Card]) -> HandResult {
        let mut result = 0;

        hand.sort_by(|a, b| a.value.value().cmp(&b.value.value()));

        let mut highest_card = hand[0];

        let mut in_a_row = 1;
        let mut prev_value = hand[0].value.value();

        if hand.last().unwrap().value == Value::Ace && highest_card.value == Value::Two {
            in_a_row = 2;
        }

        for (i, card) in hand.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if card.value.value() == prev_value + 1 {
                in_a_row = in_a_row + 1;
            } else {
                in_a_row = 1;
            }

            if in_a_row >= 5 {
                result = 4;
                highest_card = *card;
            }

            prev_value = card.value.value();
        }

        HandResult {
            result: result,
            highest: highest_card
        }
    }
}

impl Ord for HandResult {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.result > other.result {
            Ordering::Greater
        } else if self.result < other.result {
            Ordering::Less
        } else {
            self.highest.cmp(&other.highest)
        }
    }
}

impl PartialOrd for HandResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandResult {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result && self.highest == other.highest
    }
}

impl fmt::Display for HandResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.result {
            8 => {
                write!(f, "Straight Flush");
            },
            7 => {
                write!(f, "4 Of A Kind");
            },
            6 => {
                write!(f, "Full House");
            },
            5 => {
                write!(f, "Flush");
            },
            4 => {
                write!(f, "Straight");
            },
            3 => {
                write!(f, "3 Of A Kind");
            },
            2 => {
                write!(f, "Two Pair");
            },
            1 => {
                write!(f, "Pair");
            },
            0 => {
                write!(f, "High Card");
            },
            _ => {
                write!(f, "This should not occur");
            }
        }

        Ok(())
    }
}

#[test]
fn test_check_straight_flush() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Six),
        Card::new(Suit::Hearts, Value::Two),
        Card::new(Suit::Hearts, Value::Three),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Hearts, Value::Five),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 8);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Six));
}

#[test]
fn test_check_non_straight_flush() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Six),
        Card::new(Suit::Hearts, Value::Two),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Hearts, Value::Five),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 5);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ten));
}

#[test]
fn test_check_straight_flush_with_ace_low() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Hearts, Value::Two),
        Card::new(Suit::Hearts, Value::Three),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Hearts, Value::Five),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 8);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ace));
}

#[test]
fn test_check_straight_flush_with_ace_high() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Hearts, Value::King),
        Card::new(Suit::Hearts, Value::Queen),
        Card::new(Suit::Hearts, Value::Jack),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 8);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ace));
}

#[test]
fn test_check_four_of_a_kind() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Diamonds, Value::Ace),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ace),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 7);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ace));
}

#[test]
fn test_check_full_house() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Diamonds, Value::Ace),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Six),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 6);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ace));
}

#[test]
fn test_check_flush() {
    let mut cards = vec!(
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Diamonds, Value::Ace),
        Card::new(Suit::Spades, Value::Three),
        Card::new(Suit::Diamonds, Value::Seven),
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Diamonds, Value::King),
        Card::new(Suit::Diamonds, Value::Queen)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 5);
    assert_eq!(test_result.highest, Card::new(Suit::Diamonds, Value::Ace));
}

#[test]
fn test_check_straight() {
    let mut cards = vec!(
        Card::new(Suit::Hearts, Value::Three),
        Card::new(Suit::Diamonds, Value::Seven),
        Card::new(Suit::Spades, Value::Five),
        Card::new(Suit::Clubs, Value::Seven),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Diamonds, Value::King),
        Card::new(Suit::Spades, Value::Six)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 4);
    assert_eq!(test_result.highest, Card::new(Suit::Diamonds, Value::Seven));
}

#[test]
fn test_three_of_a_kind() {
    let mut cards = vec!(
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Ten),
        Card::new(Suit::Clubs, Value::Queen)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 3);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ten));
}

#[test]
fn test_two_pair() {
    let mut cards = vec!(
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Queen)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 2);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ace));
}

#[test]
fn test_pair() {
    let mut cards = vec!(
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Two),
        Card::new(Suit::Clubs, Value::Queen)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 1);
    assert_eq!(test_result.highest, Card::new(Suit::Clubs, Value::Ten));
}

#[test]
fn test_high_card() {
    let mut cards = vec!(
        Card::new(Suit::Diamonds, Value::Five),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Two),
        Card::new(Suit::Clubs, Value::Queen)
    );

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, 0);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ace));
}
