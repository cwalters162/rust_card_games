use crate::card::Card;

pub struct Hand {
    cards: Vec<Card>,
    has_ace: bool,
}

impl Hand {
    pub fn new() -> Self {
        Hand {
            cards: Vec::new(),
            has_ace: false,
        }
    }
    pub fn get_value(&self) -> usize {
        if self.has_ace == false {
            self.cards.iter().map(|card| card.get_value()).sum()
        } else {
            self.cards.iter().map(|card|
                if card.is_ace() {
                    return 11
                } else {
                    card.get_value()
                }
            ).sum()
        }
    }
    pub fn add_card(&mut self, card: Card) {
        if card.is_ace() {
            self.has_ace == true;
        }
        self.cards.push(card);
    }

    pub fn get_card(&self, index: usize) -> Card {
        self.cards.get(index).unwrap().clone()
    }
}