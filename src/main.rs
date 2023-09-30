mod clock;
mod deck;
mod card;

use crate::clock::clock;

use std::ops::IndexMut;
use std::time::Instant;
use std::io::stdin;

#[derive(PartialEq)]
pub enum GameResult {
    WON,
    LOSS,
}

fn main() {
    console_interface();
}

fn console_interface() {
    'ui_loop: loop {
        println!("What game would you like to play?");
        println!("1. Clock");
        println!("2. Blackjack");
        println!("0. Quit");

        let mut buffer: String = String::new();
        stdin().read_line(&mut buffer).unwrap();

        let res = buffer.trim().parse::<i32>();

        match res.unwrap() {
            1 => { start_clock() }
            //1 => { start_blackjack()}
            0 => {
                println!("Goodbye!");
                break 'ui_loop;
            }
            _ => {println!("I'm sorry please you only the numbers listed")}
        }
    }
}

fn start_clock() {
    let start = Instant::now();
    let mut games_won = 0.0;
    let mut games_loss = 0.0;

    for _ in 0..10000 {
        match clock() {
            GameResult::WON => games_won += 1.0,
            GameResult::LOSS => games_loss += 1.0,
        }
    }

    let game_won_percentage: f64 = games_won / games_loss * 100.0;
    println!("Clock games won: {}, loss: {}, ratio: {}%", games_won, games_loss, game_won_percentage);
    println!("Time taken to complete {} of games: {:?}", games_won + games_loss, Instant::now().duration_since(start));
}