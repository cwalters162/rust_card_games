# Blackjack

The classic game programmed in Rust!
This module provides a system to enable playing the game with your choice of interface!

Currently implemented methods of interaction

- [ ] CLI
- [ ] Desktop GUI
- [ ] Website
 
## Rules

Try to get a total of 21 points or beat the Dealer's hand.

Dealer must offer bet insurance if first card is an ACE.

### Bet Ratios

Insurance is 1.5/1.

Regular is 2/1


## Card Values

- Ace: 1 or 11
- King, Queen, Jack: 10
- All others Face Value

## Requirements

This module should provide an API that allows any frontend to interact with it 
and provide enough information to play a full hand of Blackjack.

- [ ] Interface that allows retrieval of the dealer's hand.
- [ ] Interface that allows retrieval of the player's hand.
- [ ] The ability to hit or stand as the player.
- [ ] Ability to place a bet.
- [ ] Processing of the turn information.

## API

new_game();

get_player_hand();

get_dealer_hand();

player_hit();

dealer_hit();

get_player_value();

get_dealer_value();


