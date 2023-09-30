use crate::card::Card;
use rand::prelude::*;

fn shuffle(vec: &mut Vec<Card>) {
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);
}

pub fn make_deck(isShuffled: bool) -> Vec<Card> {
    let suits = Vec::from(["Diamonds", "Hearts", "Clubs", "Spades"]);
    let names = Vec::from(["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"]);
    let values = Vec::from([1,2,3,4,5,6,7,8,9,10,11,12,13]);

    let mut cards = Vec::new();

    for suit in suits {
        for (x, _value) in values.iter().enumerate() {
            cards.push(Card::new(
                names[x].to_string(),
                suit.to_string(),
                values[x],
                false
            ))
        }
    }
    if isShuffled == true {
        shuffle(&mut cards);
    }
    cards
}