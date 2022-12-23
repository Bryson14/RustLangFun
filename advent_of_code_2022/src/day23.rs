use crate::utils::read_data;
use std::collections::HashSet;

const FILE: &str = "day23.txt";
const DAY: &str = "{{ DAY 23 }}";

/// --- Day 23: Unstable Diffusion ---
/// Using a HashSet to remember the position of the elves
pub fn part1() {
    let data = read_data(FILE);
    let mut elves: HashSet<Pos> =
        HashSet::with_capacity(data.chars().filter(|&c| c == '#').count());
    data.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, v)| {
            if v == '#' {
                elves.insert(Pos {
                    x: col as i64,
                    y: row as i64,
                });
            }
        });
    })
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
pub fn part2() {
    let data = read_data(FILE);
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}
