mod clock;
mod deck;

use std::alloc::alloc;
use crate::clock::clock;

use rand;
use rand::seq::SliceRandom;
use std::ops::IndexMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[derive(PartialEq)]
pub enum GameResult {
    WON,
    LOSS,
}

//MultiThreaded Attempt
fn main() {
    // test();
    let start = Instant::now();
    let games_won = Arc::new(Mutex::new(0.0));
    let games_loss = Arc::new(Mutex::new(0.0));
    let mut handles = vec![];

    for _ in 0..10 {
        let games_won = Arc::clone(&games_won);
        let games_loss = Arc::clone(&games_loss);

        let handle = thread::spawn(move || {
            let mut gw = games_won.lock().unwrap();
            let mut gl = games_loss.lock().unwrap();

            match clock() {
                GameResult::WON => *gw += 1.0,
                GameResult::LOSS => *gl += 1.0,
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let game_won_percentage: f64 = (*games_won.lock().unwrap() / *games_loss.lock().unwrap() * 100.0);
    println!("Clock games won: {}, loss: {}, ratio: {}%", *games_won.lock().unwrap(), *games_loss.lock().unwrap(), game_won_percentage);
    println!("Time taken to complete {} of games: {:?}", *games_won.lock().unwrap() + *games_loss.lock().unwrap(), Instant::now().duration_since(start));
}


//Single Thread
// fn main() {
//     // test();
//     let start = Instant::now();
//     let mut games_won = 0.0;
//     let mut games_loss = 0.0;
//
//     for _ in 0..10 {
//             match clock() {
//                 GameResult::WON => games_won += 1.0,
//                 GameResult::LOSS => games_loss += 1.0,
//             }
//     }
//
//     let game_won_percentage: f64 = (games_won / games_loss * 100.0);
//     println!("Clock games won: {}, loss: {}, ratio: {}%", games_won, games_loss, game_won_percentage);
//     println!("Time taken to complete {} of games: {:?}", games_won + games_loss, Instant::now().duration_since(start));
// }