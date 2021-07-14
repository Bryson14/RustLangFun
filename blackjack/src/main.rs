mod blackjack;
use blackjack::BlackJack;
use std::io;
use blackjack::get_int_input;

fn main() {
    let num_players = get_int_input(String::from("How many people are playing?"));
    let mut blackjack = BlackJack::new(num_players);
    blackjack.start();
}