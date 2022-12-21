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
    let nums = decrypt(nums, 1);
    println!("{DAY}-1 ans to number mixing in {}", get_coordinates(nums));
}

pub fn part2() {
    let data = read_data(FILE);
    let mut nums = read_nums(data);
    nums.iter_mut()
        .for_each(|data| data.val = data.val * 811589153);
    let nums = decrypt(nums, 10);
    println!("{DAY}-1 ans to number mixing in {}", get_coordinates(nums));
}

fn decrypt(mut nums: Vec<Data>, cycles: usize) -> Vec<Data> {
    let message_size = nums.len() as i64 - 1;
    for _mixing_round in 0..cycles {
        for current in 0..nums.len() {
            let index = nums.iter().position(|x| x.original_pos == current).unwrap();
            let mut new_index = index as i64 + nums[index].val;
            new_index = ((new_index % message_size) + message_size) % message_size;
            let number = nums.remove(index);
            nums.insert(new_index as usize, number);
        }
    }

    nums
}

/// Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers
///  after the value 0, wrapping around the list as necessary.
/// In the above example, the 1000th number after 0 is 4, the 2000th is -3, and the 3000th is 2;
/// adding these together produces 3.
fn get_coordinates(nums: Vec<Data>) -> i64 {
    let zero_idx = nums
        .iter()
        .position(|data| data.val == 0)
        .expect("Could not find value '0' in the list");

    let one_thousand = (zero_idx + 1000) % nums.len();
    let two_thousand = (zero_idx + 2000) % nums.len();
    let three_thousand = (zero_idx + 3000) % nums.len();

    nums[one_thousand].val + nums[two_thousand].val + nums[three_thousand].val
}

fn read_nums(data: String) -> Vec<Data> {
    data.lines()
        .enumerate()
        .map(|(i, line)| Data {
            val: line
                .trim()
                .parse()
                .expect(&format!("Could not parse {line} to i32")),
            original_pos: i,
        })
        .collect()
}

struct Data {
    val: i64,
    original_pos: usize,
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
        let data = "1
        2
        -3
        4
        0
        3
        -2";
        let nums = read_nums(data.into());
        assert_eq!(get_coordinates(nums), 3);
    }

    #[test]
    fn test_mix_numbers() {
        let data = "1
        2
        -3
        3
        -2
        0
        4";
        let nums = read_nums(data.into());
        assert_eq!(get_coordinates(nums), 3);
    }
}
