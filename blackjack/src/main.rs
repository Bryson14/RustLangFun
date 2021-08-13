mod blackjack;
mod card;
mod deck;
mod hand;
mod user;
use blackjack::get_int_input;
use blackjack::BlackJack;
mod blackjack_sim;
use blackjack_sim::BlackJackSim;
use std::io::{stdin, stdout, Read, Write};

/// This is main function that gets the number of players from the user and starts `blackjack()`
/// #![doc(html_favicon_url = "data:image/jpeg;base64,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")]
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
