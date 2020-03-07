extern crate rand;
extern crate strum;

#[macro_use]
extern crate strum_macros;

mod card;
mod player;
mod hand_result;

use card::*;
use player::*;



use strum::IntoEnumIterator;
use rand::thread_rng;
use rand::seq::SliceRandom;

const NUM_PLAYERS: usize = 8;

fn main() {
    let mut rng = thread_rng();

    let mut players = Vec::<Player>::new();
    let mut cards = Vec::<Card>::new();
    let mut table_cards = Player::new("Table".to_string());

    for suit in Suit::iter() {
        for value in Value::iter() {
            cards.push(Card::new(suit, value));
        }
    }

    println!("Shuffling!");
    cards.shuffle(&mut rng);

    for i in 0..NUM_PLAYERS {
        let player_name = format!("Player {}", i + 1);
        players.push(Player::new(player_name));
    }

    for iter in 0..NUM_PLAYERS*2 {
        let player = iter % NUM_PLAYERS;
        players[player].add_card(cards[iter]);
    }

    for iter in NUM_PLAYERS*2..NUM_PLAYERS*2+5 {
        table_cards.add_card(cards[iter])
    }

    // println!("{}", table_cards);

    for player in players.iter_mut() {
        player.add_cards(table_cards.cards.clone());
    }

    let mut winner_index = 0;
    let mut best_hand = hand_result::HandResult::check_hand(&mut players[0].cards.clone());

    for (i, player) in players.iter().enumerate() {
        let player_hand = hand_result::HandResult::check_hand(&mut player.cards.clone());
        println!("{}", player);
        println!("{}\n", player_hand);

        if player_hand > best_hand {
            println!("Current best hand: {}", player_hand);
            best_hand = player_hand;
            winner_index = i;
        }
    }

    println!("Best hand: {} for player:\n{}", best_hand, players[winner_index]);

}
