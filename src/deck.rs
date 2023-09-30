use std::fmt;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Card {
    name: String,
    suit: String,
    value: usize
}

impl Card {
    fn what_card(&self) {
        println!("I have the {} of {} and it's value is {}", self.name, self.suit, self.value)
    }

    pub fn get_value(&self) -> usize {
        self.value
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

pub fn make_deck(isShuffled: bool) -> Vec<Card> {
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