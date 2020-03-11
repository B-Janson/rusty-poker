use crate::card::*;

use self::ResultName::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
enum ResultName {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug, Eq)]
pub struct HandResult {
    result: ResultName,
    highest: Card,
}

impl HandResult {
    pub fn check_hand(hand: &mut [Card]) -> HandResult {
        if hand.len() == 0 {
            panic!("Panicked due to checking an empty hand.");
        }

        let flush_check = Self::check_flush(hand);
        if flush_check.result == StraightFlush {
            return flush_check;
        }

        let of_a_kind_check = Self::check_of_a_kind(hand);
        if of_a_kind_check.result == FourOfAKind {
            return of_a_kind_check;
        }

        if of_a_kind_check.result == FullHouse {
            return of_a_kind_check;
        }

        if flush_check.result == Flush {
            return flush_check;
        }

        let straight_check = Self::check_straight(hand);
        if straight_check.result == Straight {
            return straight_check;
        }

        if of_a_kind_check.result == ThreeOfAKind {
            return of_a_kind_check;
        }

        if of_a_kind_check.result == TwoPair {
            return of_a_kind_check;
        }

        if of_a_kind_check.result == Pair {
            return of_a_kind_check;
        }

        hand.sort_by(|a, b| a.value.value().cmp(&b.value.value()));

        return HandResult {
            result: HighCard,
            highest: *hand.last().unwrap(),
        };
    }

    fn check_flush(hand: &mut [Card]) -> HandResult {
        let mut result = HighCard;
        let mut highest = hand[0];

        // Make a hashmap to keep track of the number of cards of the same suit.
        let mut card_map = HashMap::<Suit, u32>::new();

        // For each card
        for card in hand.iter() {
            // If this card's suit is already in the map, increment its count and
            // re-insert
            *card_map.entry(card.suit).or_insert(0) += 1;
        }

        for (s, n) in card_map.into_iter() {
            if n >= 5 {
                let mut flush_cards = hand
                    .iter()
                    .filter(|&c| c.suit == s)
                    .cloned()
                    .collect::<Vec<Card>>();

                let test = Self::check_straight(&mut flush_cards);

                if test.result == Straight {
                    result = StraightFlush;
                } else {
                    result = Flush;
                }

                highest = *flush_cards.last().unwrap();
            }
        }

        HandResult {
            result: result,
            highest: highest,
        }
    }

    fn check_of_a_kind(hand: &mut [Card]) -> HandResult {
        let mut result = HighCard;
        let mut highest = hand[0];

        // Make a hashmap to keep track of the number of cards of the same value.
        let mut card_map = HashMap::<Value, Vec<Card>>::new();

        // For each card
        for card in hand.iter() {
            // If this card's suit is already in the map, increment its count and
            // re-insert
            card_map.entry(card.value).or_insert(vec![]).push(*card);
        }

        let mut final_cards = Vec::<Card>::new();

        for (v, n) in card_map.into_iter() {
            if n.len() == 4 {
                return HandResult {
                    result: FourOfAKind,
                    highest: hand
                        .iter()
                        .filter(|&c| c.value == v)
                        .cloned()
                        .collect::<Vec<Card>>()[0],
                };
            }

            if n.len() > 1 {
                final_cards.extend(n);
            }
        }

        final_cards.sort_by(|a, b| a.value.value().cmp(&b.value.value()));

        if final_cards.len() == 5 || final_cards.len() == 7 {
            result = FullHouse;
            highest = *final_cards.last().unwrap();
        } else if final_cards.len() == 4 || final_cards.len() == 6 {
            result = TwoPair;
            highest = *final_cards.last().unwrap();
        } else if final_cards.len() == 3 {
            result = ThreeOfAKind;
            highest = *final_cards.last().unwrap();
        } else if final_cards.len() == 2 {
            result = Pair;
            highest = *final_cards.last().unwrap();
        }

        HandResult {
            result: result,
            highest: highest,
        }
    }

    fn check_straight(hand: &mut [Card]) -> HandResult {
        let mut result = HighCard;

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
                result = Straight;
                highest_card = *card;
            }

            prev_value = card.value.value();
        }

        HandResult {
            result: result,
            highest: highest_card,
        }
    }
}

impl ResultName {
    fn value(&self) -> u32 {
        match *self {
            StraightFlush => 8,
            FourOfAKind => 7,
            FullHouse => 6,
            Flush => 5,
            Straight => 4,
            ThreeOfAKind => 3,
            TwoPair => 2,
            Pair => 1,
            HighCard => 0,
        }
    }
}

impl Ord for HandResult {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.result.value() > other.result.value() {
            Ordering::Greater
        } else if self.result.value() < other.result.value() {
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
            StraightFlush => {
                write!(f, "Straight Flush")?;
            }
            FourOfAKind => {
                write!(f, "4 Of A Kind")?;
            }
            FullHouse => {
                write!(f, "Full House")?;
            }
            Flush => {
                write!(f, "Flush")?;
            }
            Straight => {
                write!(f, "Straight")?;
            }
            ThreeOfAKind => {
                write!(f, "3 Of A Kind")?;
            }
            TwoPair => {
                write!(f, "Two Pair")?;
            }
            Pair => {
                write!(f, "Pair")?;
            }
            HighCard => {
                write!(f, "High Card")?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_check_straight_flush() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Six),
        Card::new(Suit::Hearts, Value::Two),
        Card::new(Suit::Hearts, Value::Three),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Hearts, Value::Five),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, StraightFlush);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Six));
}

#[test]
fn test_check_non_straight_flush() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Six),
        Card::new(Suit::Hearts, Value::Two),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Hearts, Value::Five),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, Flush);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ten));
}

#[test]
fn test_check_straight_flush_with_ace_low() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Hearts, Value::Two),
        Card::new(Suit::Hearts, Value::Three),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Hearts, Value::Five),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, StraightFlush);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ace));
}

#[test]
fn test_check_straight_flush_with_ace_high() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Hearts, Value::King),
        Card::new(Suit::Hearts, Value::Queen),
        Card::new(Suit::Hearts, Value::Jack),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, StraightFlush);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ace));
}

#[test]
fn test_check_four_of_a_kind() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Diamonds, Value::Ace),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ace),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, FourOfAKind);
    assert_eq!(test_result.highest, Card::new(Suit::Hearts, Value::Ace));
}

#[test]
fn test_check_full_house() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Diamonds, Value::Ace),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Six),
        Card::new(Suit::Hearts, Value::Ten),
        Card::new(Suit::Diamonds, Value::Two),
        Card::new(Suit::Spades, Value::Two),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, FullHouse);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ace));
}

#[test]
fn test_check_flush() {
    let mut cards = vec![
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Diamonds, Value::Ace),
        Card::new(Suit::Spades, Value::Three),
        Card::new(Suit::Diamonds, Value::Seven),
        Card::new(Suit::Hearts, Value::Ace),
        Card::new(Suit::Diamonds, Value::King),
        Card::new(Suit::Diamonds, Value::Queen),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, Flush);
    assert_eq!(test_result.highest, Card::new(Suit::Diamonds, Value::Ace));
}

#[test]
fn test_check_straight() {
    let mut cards = vec![
        Card::new(Suit::Hearts, Value::Three),
        Card::new(Suit::Diamonds, Value::Seven),
        Card::new(Suit::Spades, Value::Five),
        Card::new(Suit::Clubs, Value::Seven),
        Card::new(Suit::Hearts, Value::Four),
        Card::new(Suit::Diamonds, Value::King),
        Card::new(Suit::Spades, Value::Six),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, Straight);
    assert_eq!(test_result.highest, Card::new(Suit::Diamonds, Value::Seven));
}

#[test]
fn test_three_of_a_kind() {
    let mut cards = vec![
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Ten),
        Card::new(Suit::Clubs, Value::Queen),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, ThreeOfAKind);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ten));
}

#[test]
fn test_two_pair() {
    let mut cards = vec![
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Queen),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, TwoPair);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ace));
}

#[test]
fn test_pair() {
    let mut cards = vec![
        Card::new(Suit::Diamonds, Value::Ten),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Two),
        Card::new(Suit::Clubs, Value::Queen),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, Pair);
    assert_eq!(test_result.highest, Card::new(Suit::Clubs, Value::Ten));
}

#[test]
fn test_high_card() {
    let mut cards = vec![
        Card::new(Suit::Diamonds, Value::Five),
        Card::new(Suit::Spades, Value::Ace),
        Card::new(Suit::Clubs, Value::Ten),
        Card::new(Suit::Hearts, Value::Seven),
        Card::new(Suit::Diamonds, Value::Four),
        Card::new(Suit::Spades, Value::Two),
        Card::new(Suit::Clubs, Value::Queen),
    ];

    let test_result = HandResult::check_hand(&mut cards);

    assert_eq!(test_result.result, HighCard);
    assert_eq!(test_result.highest, Card::new(Suit::Spades, Value::Ace));
}
