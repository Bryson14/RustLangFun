use super::input;
use super::part1::{calculate_ways_to_win, calculate_wins_by_roots};
use std::time::Instant;


/// # --- Part Two ---
/// As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.
/// 
/// So, the example from before:
/// 
/// Time:      7  15   30
/// Distance:  9  40  200
/// ...now instead means this:
/// 
/// Time:      71530
/// Distance:  940200
/// Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!
/// 
/// How many ways can you beat the record in this one much longer race?
pub fn solve(input: &str) {
    let race = input::read_race_part_2(input);

    let start = Instant::now();
    let answer = calculate_ways_to_win(&race);
    let duration = start.elapsed();
    let start = Instant::now();
    let answer2 = calculate_wins_by_roots(&race);
    let duration2 = start.elapsed();

    println!("Ways to win: {}", answer);
    println!("Ways to win by roots: {}", answer2);
    println!("Time to calculate ways to win: {:?}", duration);
    println!("Time to calculate ways to win by roots: {:?}", duration2);
}
