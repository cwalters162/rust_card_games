use rand;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone)]
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

fn shuffle(coll: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    coll.shuffle(&mut rng);
}

fn make_cards(isShuffled: bool) -> Vec<Card> {
    let suits = vec!["Diamonds", "Hearts", "Clubs", "Spades"];
    let names = vec!["Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
    let values = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];

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

// Start of the functions for clock

fn clock() -> bool {
    let mut game_complete = false;
    let mut cards = make_cards(true);
    let mut split_cards: Vec<Vec<Card>> = cards.chunks(4).map(|c| c.into()).collect();

    while game_complete != true {
        let mut current_card= split_cards.pop().unwrap().pop();
        println!("Current Card {}", current_card.unwrap());
        game_complete = true;
    }
    true
}

fn main() {
    let mut cards = make_cards(true);

    for card in cards {
        card.what_card();
        println!("Current Card {}", card);
    }
}