#![allow(unused)]
use std::{fs::read, intrinsics::unreachable};

use crate::utils::read_data;

const FILE: &str = "day17.txt";
const DAY: &str = "{{ DAY 17 }}";

pub fn part1() {
    let data = read_data(FILE);
    let jet_blasts = read_air_jets(data);
}

pub fn part2() {
    let data = read_data(FILE);
}

enum TetrisPiece {
    Horizontal,
    Cross,
    Elbow,
    Vertical,
    Square,
}

enum AirDirection {
    Left,
    Right,
}

fn read_air_jets(data: String) -> Vec<AirDirection> {
    data.trim()
        .chars()
        .map(|c| match c {
            '<' => AirDirection::Left,
            '>' => AirDirection::Right,
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
