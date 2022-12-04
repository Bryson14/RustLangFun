use crate::utils::read_data;

const FILE: &str = "day4.txt";
const DAY: &str = "{{ DAY 4 }}";

struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn new(low: usize, high: usize) -> Self {
        assert!(low <= high);
        Range { low, high }
    }
}

fn read_pair(line: &str) -> Vec<Range> {
    line.split(',')
        .map(|r| {
            r.split('-')
                .map(|d| d.parse::<usize>().expect("cannot parse {d}"))
                .collect::<Vec<usize>>()
        })
        .map(|digits| Range::new(digits[0], digits[1]))
        .collect()
}

fn is_fully_contained(r1: &Range, r2: &Range) -> bool {
    // r1 contains r2
    (r1.low <= r2.low && r1.high >= r2.high) || (r2.low <= r1.low && r2.high >= r1.high)
}

fn is_overlapped(r1: &Range, r2: &Range) -> bool {
    (r1.low <= r2.high && r2.low <= r1.low) || (r2.low <= r1.high && r1.low <= r2.low)
}

/// --- Day 4: Camp Cleanup ---
/// In how many assignment pairs does one range fully contain the other?
pub fn part1() {
    let data = read_data(FILE);
    let count_contained = data
        .lines()
        .map(read_pair)
        .map(|pairs| is_fully_contained(&pairs[0], &pairs[1]))
        .filter(|&contained| contained)
        .count();
    println!("{DAY}-1 There are {count_contained} pairs that contain each other");
}

/// In how many assignment pairs do the ranges overlap?
pub fn part2() {
    let data = read_data(FILE);
    let count_contained = data
        .lines()
        .map(read_pair)
        .map(|pairs| is_overlapped(&pairs[0], &pairs[1]))
        .filter(|&contained| contained)
        .count();
    println!("{DAY}-2 There are {count_contained} pairs that overlap each other");
}
