use std::collections::VecDeque;
use rand;
use rand::seq::SliceRandom;
use std::fmt;
use std::time::{Duration};
use std::thread;

#[derive(Clone, Debug)]
struct Card {
    name: String,
    suit: String,
    value: usize
}

impl Card {
    fn what_card(&self) {
        println!("I have the {} of {} and it's value is {}", self.name, self.suit, self.value)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The {} of {}", self.name, self.suit)
    }
}

fn shuffle(vec: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);
}

fn make_cards(isShuffled: bool) -> Vec<Card> {
    let suits = Vec::from(["Diamonds", "Hearts", "Clubs", "Spades"]);
    let names = Vec::from(["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"]);
    let values = Vec::from([1,2,3,4,5,6,7,8,9,10,11,12,13]);

    let mut cards = Vec::new();

    for suit in suits {
        for (x, _value) in values.iter().enumerate() {
            cards.push(Card {
                name: names[x].to_string(),
                suit: suit.to_string(),
                value: values[x]
            })
        }
    }
    if isShuffled == true {
        shuffle(&mut cards);
    }
    cards
}

fn test() {
    let cards = make_cards(true);

    for card in cards {
        card.what_card();
        println!("Current Card {}", card);
    }
}

// Start of the functions for clock
/*
Game creates a deck of shuffled cards
Splits the cards into 13 piles.
Take the 1st card from the 13th pile.
then swaps cards until the 13th pile is complete or all piles have matching cards.
*/
fn clock() -> bool {
    let time_to_view_cards = Duration::from_secs(2);
    let mut game_complete = false;
    let cards = make_cards(true);

    let mut split_cards: Vec<VecDeque<Card>> = cards.chunks(4).map(|card| card.iter().cloned().collect()).collect();

    //get the first card from the 13th pile
    let mut current_card = split_cards.last_mut().map(|chunk| chunk.pop_back().unwrap()).unwrap();

    while game_complete != true {
        println!("Current card: {}", current_card);

        for (i, chunk) in split_cards.iter_mut().enumerate() {
            //insert it into it's correct chunk.
            println!("I: {}.... current card value: {}", &i, &current_card.value);

            if i == current_card.value {
                println!("Inside of the if statement");
                chunk.push_front(current_card.clone());
                current_card = chunk.pop_back().unwrap();
            }
            //repeat until 13th pile is done.
            // break out if the 13th pile has 4 of the same value.
            if i == 12 {
                let mut total_cards = 0;
                let _ = chunk.iter().map(|card| if card.value == 12 { total_cards += 1; });
                if total_cards == 4 {
                    game_complete = true;
                    break;
                }
            }
            //get the next card from that chunk.

        }


        // check if all chunks have the same value.
        //if true game is won' else lost.
        thread::sleep(time_to_view_cards);
        // game_complete = true;
    }
    true
}



fn main() {
    // test();
    clock();
}