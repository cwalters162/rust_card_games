use crate::card::Card;
use crate::deck::make_deck;
use crate::GameResult;
use crate::hand::Hand;

pub struct Blackjack {
    deck: Vec<Card>,
    player_hand: Hand,
    dealer_hand: Hand,
    money: isize,
    bet: usize,
}

impl Blackjack {
    pub fn new_game() -> Blackjack {
        Blackjack {
            deck: make_deck(true),
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
            money: 100,
            bet: 0,
        }
    }

    pub fn get_player_hand(&self) -> &Hand {
        &self.player_hand
    }

    pub fn get_dealer_hand(&self) -> &Hand {
        &self.dealer_hand
    }

    pub fn player_hit(&mut self) -> &Hand {
        self.player_hand.add_card(self.deck.pop().unwrap());
        &self.player_hand
    }

    pub fn dealer_hit(&mut self) -> &Hand {
        self.dealer_hand.add_card(self.deck.pop().unwrap());
        &self.dealer_hand
    }

    pub fn get_player_value(&self) -> usize {
        self.player_hand.get_value()
    }

    pub fn get_dealer_value(&self) -> usize {
        self.dealer_hand.get_value()
    }
    //returns true if bet was set successfully otherwise false.
    pub fn set_bet(&mut self, bet: usize) -> bool {
        if bet as isize > self.money {
            false
        } else {
            self.bet = bet;
            self.money -= bet as isize;
            true
        }
    }
}

pub fn start_blackjack() -> GameResult {
    //create a new game of blackjack
    let mut game = Blackjack::new_game();
    //get player bet
    game.set_bet(10);
    //deal cards to the dealer
    game.dealer_hit();
    if game.get_dealer_hand().get_card(0).is_ace() {
        //offer bet insurance
    }
    game.dealer_hit();
    //hit the player twice
    game.player_hit();
    game.player_hit();
    //display player hand + value
    while game.get_player_value() < 17 {
        game.player_hit();
    }

    if game.get_player_value() > 21 {
        return GameResult::LOSS;
    }

    while game.get_dealer_value() < 17 {
        game.dealer_hit();
    }

    if game.get_dealer_value() > 21 {
        return GameResult::WON;
    }

    if game.get_dealer_value() >= game.get_player_value() {
        GameResult::LOSS
    } else {
        GameResult::WON
    }
}