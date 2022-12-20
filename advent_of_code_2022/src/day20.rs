#![allow(unused)]
use std::collections::{HashSet, VecDeque};

use crate::utils::read_data;

const FILE: &str = "day20.txt";
const DAY: &str = "{{ DAY 20 }}";

/// --- Day 20: Grove Positioning System ---
/// Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th
/// numbers after the value 0, wrapping around the list as necessary.
/// In the above example, the 1000th number after 0 is 4, the 2000th is -3, and
/// the 3000th is 2; adding these together produces 3.
///
/// Mix your encrypted file exactly once. What is the sum of the three numbers that form the grove coordinates?
pub fn part1() {
    let data = read_data(FILE);
    let nums = read_nums(data);
    let nums_set: HashSet<&i32> = HashSet::from_iter(nums.iter());

    assert_eq!(nums.len(), nums_set.len());
}

pub fn part2() {
    let data = read_data(FILE);
}

fn get_coordinates(nums: Vec<i32>) -> i32 {
    todo!()
}

fn read_nums(data: String) -> Vec<i32> {
    data.lines().map(|line| line.parse().unwrap()).collect()
}

struct Data {
    val: i32,
    move_pos: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Then, the grove coordinates can be found by looking at the 1000th, 2000th,
    /// and 3000th numbers after the value 0, wrapping around the list as necessary.
    /// In the above example, the 1000th number after 0 is 4, the 2000th is -3, and the 3000th is 2;
    /// adding these together produces 3.
    #[test]
    fn test_get_coord() {
        let nums: Vec<i32> = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(get_coordinates(nums), 3);
    }
}
