use std::collections::VecDeque;
use rand;
use rand::seq::SliceRandom;
use std::fmt;
use std::ops::IndexMut;
use std::time::{Duration, Instant};

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

#[derive(PartialEq)]
enum GameResult {
    WON,
    LOSS,
}

// Start of the functions for clock
/*
Game creates a deck of shuffled cards
Splits the cards into 13 piles.
Take the 1st card from the 13th pile.
then swaps cards until the 13th pile is complete or all piles have matching cards.
*/
fn clock() -> GameResult {
    let mut won_or_loss = GameResult::WON;
    let time_to_view_cards = Duration::from_secs(2);
    let mut game_complete = false;
    let mut cards = make_cards(true);
    let mut split_cards: Vec<VecDeque<Card>> = Vec::new();

    for _ in 0..13 {
        split_cards.push(VecDeque::new());
        for _ in 0..4 {
            split_cards.last_mut().unwrap().push_front(cards.pop().unwrap());
        }
    }

    // let mut split_cards: Vec<VecDeque<Card>> = cards.chunks(4).map(|card| card.iter().cloned().collect()).collect();

    //get the first card from the 13th pile
    let mut current_card = split_cards.last_mut().map(|chunk| chunk.pop_back().unwrap()).unwrap();

    let mut round = 0;
    let mut times_seen_king= 0;
    while game_complete != true {
        /*
        check the value of the current card
        find where it's place is
        insert that card to the front
        take the next card from the back
        repeat until all four kings/13s are in their pile.

        check if all the cards are in the rights spot game is won
        else the game is lost.
         */
        round += 1;
        //println!("Round {}. Current Card: {}", &round, &current_card);

        let current_card_value = current_card.value;
        split_cards.index_mut(current_card_value - 1).push_front(current_card);
        current_card = split_cards.index_mut(current_card_value - 1).pop_back().unwrap();
        if current_card.value == 13 {
            times_seen_king += 1
        }

        if times_seen_king == 4 {
            split_cards.index_mut(current_card_value - 1).push_front(current_card.clone());
            game_complete = true;

            for (i, pile) in split_cards.iter().enumerate() {
                for card in pile.iter() {
                    if card.value - 1 != i {
                        won_or_loss = GameResult::LOSS;
                    }
                }
            }
        }
    }
    won_or_loss
}


fn main() {
    // test();
    let start = Instant::now();
    let mut games_won: f64 = 0.0;
    let mut games_loss: f64 = 0.0;

    for _ in 0..10000000 {
        match clock() {
            GameResult::WON => games_won += 1.0,
            GameResult::LOSS => games_loss += 1.0,
        }
    }
    let game_won_percentage: f64 = (games_won / games_loss * 100.0);
    println!("Clock games won: {}, loss: {}, ratio: {}%", games_won, games_loss, game_won_percentage);
    println!("Time taken to complete {} of games: {:?}", games_won + games_loss, Instant::now().duration_since(start));
}