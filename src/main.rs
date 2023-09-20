mod clock;
mod deck;
use crate::clock::clock;

use rand;
use rand::seq::SliceRandom;
use std::ops::IndexMut;
use std::time::Instant;

#[derive(PartialEq)]
pub enum GameResult {
    WON,
    LOSS,
}

fn main() {
    // test();
    let start = Instant::now();
    let mut games_won: f64 = 0.0;
    let mut games_loss: f64 = 0.0;

    for _ in 0..1000000 {
        match clock() {
            GameResult::WON => games_won += 1.0,
            GameResult::LOSS => games_loss += 1.0,
        }
    }
    let game_won_percentage: f64 = (games_won / games_loss * 100.0);
    println!("Clock games won: {}, loss: {}, ratio: {}%", games_won, games_loss, game_won_percentage);
    println!("Time taken to complete {} of games: {:?}", games_won + games_loss, Instant::now().duration_since(start));
}