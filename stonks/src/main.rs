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

fn _partition() {
    let strings = vec!["LGR", "22", "7"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("{:?} {:?}", numbers, errors);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("{:?} {:?}", numbers, errors);
}
