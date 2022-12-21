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
    let nums_set: HashSet<i32> = HashSet::from_iter(nums.iter().map(|data| data.val));

    assert_eq!(nums.len(), nums_set.len());
}

pub fn part2() {
    let data = read_data(FILE);
}

fn mix_numbers(nums: Vec<Data>) -> Vec<Data> {
    // get idx of next in line
    for i in 0..nums.len() {
        let data_idx = nums.iter().position(|data| data.move_pos == i).unwrap();
        let data = nums.remove(data_idx);
        let insert_idx = data_idx;
        if data.val > data_idx as i32 {
            let diff = data.val - data_idx as i32;

            insert_idx = nums.len() - (data)
        }
    }
    // find loc to move it to
    // rm and insert val
    todo!()
}

/// Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers
///  after the value 0, wrapping around the list as necessary.
/// In the above example, the 1000th number after 0 is 4, the 2000th is -3, and the 3000th is 2;
/// adding these together produces 3.
fn get_coordinates(nums: Vec<Data>) -> i32 {
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
            move_pos: i,
        })
        .collect()
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
