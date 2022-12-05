use crate::utils::read_data;
use std::{collections::HashSet, hash::Hash};

const FILE: &str = "day3.txt";
const DAY: &str = "{{ DAY 3 }}";

fn char_priority(c: char) -> usize {
    match c {
        'a'..='z' => 1 + c as usize - 'a' as usize,
        'A'..='Z' => 27 + c as usize - 'A' as usize,
        ' ' => 0,
        _ => unreachable!(),
    }
}

fn similar_items(sect1: &str, sect2: &str) -> usize {
    sect1
        .chars()
        .find(|c| sect2.contains(*c))
        .map(|c| char_priority(c))
        .unwrap()
}

fn similar_items_3(sect1: &str, sect2: &str, sect3: &str) -> usize {
    sect1
        .chars()
        .find(|c| sect2.contains(*c) && sect3.contains(*c))
        .map(|c| char_priority(c))
        .unwrap()
}

/// --- Day 3: Rucksack Reorganization ---
pub fn part1() {
    let data = read_data(FILE);
    let score: usize = data
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(l, r)| similar_items(l, r))
        .sum();
    println!("{DAY}-1 The total score of matching items in rucksacks is {score}");
}

pub fn part2() {
    let data = read_data(FILE);
    let strings: Vec<&str> = data.lines().collect();
    let mut score = 0;
    for group in strings.chunks(3) {
        score += similar_items_3(
            group.to_owned()[0],
            group.to_owned()[1],
            group.to_owned()[2],
        )
    }
    println!("{DAY} The total score of matching items in rucksacks is {score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_score() {
        let data: String = "vJrwpWtwJgWrhcsFMMfFFhFp".into();
        let score: usize = data
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(l, r)| similar_items(l, r))
            .sum();
        assert_eq!(score, 16);
    }

    #[test]
    fn test_calc_score2() {
        let data: String = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into();
        let score: usize = data
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(l, r)| similar_items(l, r))
            .sum();
        assert_eq!(score, 38);
    }

    #[test]
    fn test_calc_score3() {
        let data: String = "PmmdzqPrVvPwwTWBwg".into();
        let score: usize = data
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(l, r)| similar_items(l, r))
            .sum();
        assert_eq!(score, 42);
    }

    #[test]
    fn test_calc_score4() {
        let data: String = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into();
        let score: usize = data
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(l, r)| similar_items(l, r))
            .sum();
        assert_eq!(score, 22);
    }

    #[test]
    fn test_calc_score5() {
        let data: String = "ttgJtRGJQctTZtZT".into();
        let score: usize = data
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(l, r)| similar_items(l, r))
            .sum();
        assert_eq!(score, 20);
    }

    #[test]
    fn test_calc_score6() {
        let data: String = "CrZsJsPPZsGzwwsLwLmpwMDw".into();
        let score: usize = data
            .lines()
            .map(|line| line.split_at(line.len() / 2))
            .map(|(l, r)| similar_items(l, r))
            .sum();
        assert_eq!(score, 19);
    }
}
