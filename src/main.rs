extern crate rand;
extern crate strum;
extern crate strum_macros;

mod card;
mod hand_result;
mod player;

use card::*;
use player::*;

use std::error::Error;
use std::process;
use std::time::Instant;

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;
use strum::IntoEnumIterator;

const NUM_PLAYERS: usize = 8;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    player_number: usize,
    trial_number: usize,
    card_1: Card,
    card_2: Card,
    card_3: Card,
    card_4: Card,
    card_5: Card,
    card_6: Card,
    card_7: Card,
    winner: bool,
    result_name: Option<String>,
}

fn run() -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    let mut rng = thread_rng();

    let file_path = "out.csv";
    let mut wtr = csv::Writer::from_path(file_path)?;

    for trial_number in 1..1000 {
        let mut players = Vec::<Player>::new();
        let mut cards = Vec::<Card>::new();
        let mut table_cards = Player::new("Table".to_string());

        for suit in Suit::iter() {
            for value in Value::iter() {
                cards.push(Card::new(suit, value));
            }
        }

        // println!("Shuffling!");
        cards.shuffle(&mut rng);

        for i in 0..NUM_PLAYERS {
            let player_name = format!("Player {}", i + 1);
            players.push(Player::new(player_name));
        }

        for iter in 0..NUM_PLAYERS * 2 {
            let player = iter % NUM_PLAYERS;
            players[player].add_card(cards[iter]);
        }

        for iter in NUM_PLAYERS * 2..NUM_PLAYERS * 2 + 5 {
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
            // println!("{}", player);
            // println!("{}\n", player_hand);

            if player_hand > best_hand {
                // println!("Current best hand: {}", player_hand);
                best_hand = player_hand;
                winner_index = i;
            }
        }

        // println!(
        //     "Best hand: {} for player:\n{}",
        //     best_hand, players[winner_index]
        // );

        for (i, player) in players.iter().enumerate() {
            let result_text;
            if i == winner_index {
                result_text = Some(format!("{}", best_hand));
            } else {
                result_text = None;
            }
            wtr.serialize(Record {
                player_number: i,
                trial_number: trial_number,
                card_1: player.cards[0],
                card_2: player.cards[1],
                card_3: player.cards[2],
                card_4: player.cards[3],
                card_5: player.cards[4],
                card_6: player.cards[5],
                card_7: player.cards[6],
                winner: i == winner_index,
                result_name: result_text,
            })?;
        }
    }

    wtr.flush()?;

    println!("{}", now.elapsed().as_micros());

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
