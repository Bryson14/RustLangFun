mod data_tools;
mod day1;
mod day2;

use data_tools::{read_from_data_dir, string_to_vec_i32};

fn main() {
    // # Day 1
    day1::part1();
    day1::part2();

    // # Day 2
    day2::part1();
    day2::part2();
}
