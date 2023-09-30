use crate::card::Card;

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: vec![],
        }
    }
    pub fn get_value(&self) -> usize {
        self.cards.iter()
            .map(|card| card.get_value())
            .sum()
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}