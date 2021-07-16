mod blackjack;
use blackjack::BlackJack;
use blackjack:: Hand;
use blackjack::Card;
use blackjack::get_int_input;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    let num_players = get_int_input(String::from("How many people are playing?"));
    let mut blackjack = BlackJack::new(num_players);
    blackjack.start();
    pause();
    
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to end program...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}