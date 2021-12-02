mod advent_of_code;
mod battleship;
mod filecontroler;
mod fizzbuzz;
mod hello;

use advent_of_code::adventofcode_day1::num_of_increases;
use battleship::play_battleship;
use fizzbuzz::fizzbuzz;
use hello::greet;

fn main() {
    // greet("Bryson".to_owned());
    // separator();
    // fizzbuzz(15);
    // separator();
    // play_battleship();
    num_of_increases();
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
