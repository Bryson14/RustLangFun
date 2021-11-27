mod battleship;
mod filecontroler;
mod fizzbuzz;
mod hello;

use battleship::play_battleship;
use filecontroler::{append_file, open_file};
use fizzbuzz::fizzbuzz;
use hello::greet;

fn main() {
    greet("Bryson".to_owned());
    separator();
    fizzbuzz(15);
    // separator();
    // append_file("message.txt", "\nThis is stupid");
    // open_file("message.txt");
    separator();
    play_battleship();
}

fn separator() {
    let line = (0..50).map(|_x| "*").collect::<Vec<&str>>().join("");
    println!("\n{}\n", line);
}
