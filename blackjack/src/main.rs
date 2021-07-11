mod blackjack;
use blackjack::BlackJack;
use std::io;

fn main() {
    let num_players = get_int_input(String::from("How many people are playing?"));
    let mut blackjack = BlackJack::new(num_players);
    println!("{}", blackjack.deck.draw());
    blackjack.start();
    
}

fn get_int_input(message: String) -> isize {
    println!("{}", message);
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
        
        let var: isize = match var.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you messed up idiot! Try Again!");
                get_int_input(message)
            },
        };
    var
}