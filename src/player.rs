use crate::card::Card;

use std::fmt;

pub struct Player {
    pub cards: Vec<Card>,
    name: String
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            cards: Vec::<Card>::new(),
            name: name
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            self.add_card(card);
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} has:", self.name)?;
        for card in &self.cards {
            writeln!(f, "\t- {}", card)?;
        }
        Ok(())
    }
}
