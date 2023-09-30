use std::fmt;

#[derive(Clone, Debug)]
pub struct Card {
    name: String,
    suit: String,
    value: usize,
    is_ace: bool,
}

impl Card {
    pub fn new(name: String, suit: String, value: usize, is_ace: bool) -> Card {
        Card {
            name,
            suit,
            value,
            is_ace,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_suit(&self) -> String {
        self.suit.clone()
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn is_ace(&self) -> bool {
        self.is_ace
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The {} of {}", self.name, self.suit)
    }
}