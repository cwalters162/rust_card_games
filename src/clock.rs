use crate::deck::make_deck;
use crate::card::Card;
use crate::GameResult;

use std::collections::VecDeque;
use std::ops::IndexMut;
use std::time::{Duration};



// Start of the functions for clock
/*
Game creates a deck of shuffled cards
Splits the cards into 13 piles.
Take the 1st card from the 13th pile.
then swaps cards until the 13th pile is complete or all piles have matching cards.
*/
pub fn clock() -> GameResult {
    let mut won_or_loss = GameResult::WON;
    let mut game_complete = false;
    let mut cards = make_deck(true);
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

        let current_card_value = current_card.get_value();
        split_cards.index_mut(current_card_value - 1).push_front(current_card);
        current_card = split_cards.index_mut(current_card_value - 1).pop_back().unwrap();
        if current_card.get_value() == 13 {
            times_seen_king += 1
        }

        if times_seen_king == 4 {
            split_cards.index_mut(current_card_value - 1).push_front(current_card.clone());
            game_complete = true;

            for (i, pile) in split_cards.iter().enumerate() {
                for card in pile.iter() {
                    if card.get_value() - 1 != i {
                        won_or_loss = GameResult::LOSS;
                    }
                }
            }
        }
    }
    won_or_loss
}