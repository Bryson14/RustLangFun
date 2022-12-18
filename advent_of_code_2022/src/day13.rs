use crate::utils::read_data;
use itertools::Itertools;

const FILE: &str = "day13.txt";
const DAY: &str = "{{ DAY 13 }}";

/// --- Day 13: Distress Signal ---
/// You climb the hill and again try contacting the Elves. However, you instead receive a
/// signal you weren't expecting: a distress signal.
///
/// Your handheld device must still not be working properly; the packets
/// from the distress signal got decoded out of order. You'll need to re-order
/// the list of received packets (your puzzle input) to decode the message.
///
/// Your list consists of pairs of packets; pairs are separated by a blank line.
/// You need to identify how many pairs of packets are in the right order.
pub fn part1() {
    let data = read_data(FILE);
    let ans = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| DataStr::from_str(line))
        .tuples()
        .enumerate()
        .filter(|(_, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!(
        "{DAY} Sum of indexes of correctly ordered pairs are {}",
        ans,
    );
}

/// Now, you just need to put all of the packets in the right order.
/// Disregard the blank lines in your list of received packets.
///
/// Afterward, locate the divider packets. To find the decoder key for this distress signal,
/// you need to determine the indices of the two divider packets and multiply them together.
/// (The first packet is at index 1, the second packet is at index 2, and so on.)
/// In this example, the divider packets are 10th and 14th, and so the decoder key is 140.
///
/// Organize all of the packets into the correct order. What is the decoder key for the distress signal?
///
/// The distress signal protocol also requires that you include two additional divider packets:
pub fn part2() {
    let data = read_data(FILE);
    let divider2 = DataStr::from_str("[[2]]");
    let divider6 = DataStr::from_str("[[6]]");
    let lines = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| DataStr::from_str(line));

    let mut divider2idx = 1;
    let mut divider6idx = 2;
    for l in lines {
        if l < divider2 {
            divider2idx += 1;
            divider6idx += 1;
        } else if l < divider6 {
            divider6idx += 1;
        }
    }

    println!(
        "{DAY} Multiplied indexes of divider packets is {}",
        divider6idx * divider2idx,
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DataStr<'a>(&'a [u8]);

impl<'a> DataStr<'a> {
    fn from_str(s: &'a str) -> Self {
        Self(s.as_bytes())
    }

    fn get_int(&'a self, idx: usize) -> (u8, usize) {
        match (self.0[idx], self.0[idx + 1]) {
            (b'1', b'0') => (10, idx + 2),
            (c, _) => (c - b'0', idx + 1),
        }
    }
}

impl<'a> PartialOrd for DataStr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for DataStr<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut self_idx = 1;
        let mut self_nesting = 0;
        let mut other_idx = 1;
        let mut other_nesting = 0;

        // '[' , ']', ' ', ',', 'd'

        loop {
            match (self.0[self_idx], other.0[other_idx]) {
                (b']', b']') => {
                    self_idx += 1;
                    other_idx += 1;
                }
                (b',' | b']', _) if self_nesting > 0 => {
                    self_nesting -= 1;
                    if self_nesting == 0 && self.0[self_idx] == b',' {
                        return std::cmp::Ordering::Less;
                    }
                }
                (_, b',' | b']') if other_nesting > 0 => {
                    other_nesting -= 1;
                    if other_nesting == 0 && other.0[other_idx] == b',' {
                        return std::cmp::Ordering::Greater;
                    }
                }

                (_, b']') => return std::cmp::Ordering::Greater,
                (b']', _) => return std::cmp::Ordering::Less,
                (b'0'..=b'9', b'0'..=b'9') => {
                    let (left, left_idx) = self.get_int(self_idx);
                    let (right, right_idx) = other.get_int(other_idx);
                    match left.cmp(&right) {
                        std::cmp::Ordering::Equal => {
                            self_idx = left_idx;
                            other_idx = right_idx;
                            continue;
                        }
                        ord => return ord,
                    }
                }
                (l, r) if l == r => {
                    self_idx += 1;
                    other_idx += 1;
                }
                (b'[', _) => {
                    self_idx += 1;
                    other_nesting += 1;
                }
                (_, b'[') => {
                    other_idx += 1;
                    self_nesting += 1;
                }
                (l, r) => panic!(
                    "Found {l} ({self_idx}) and {r} ({other_idx}) for {self:?} and {other:?}"
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_1() {
        let left = "[1, 1, 3, 1, 1]";
        let right = "[1, 1, 5, 1, 1]";

        assert!(DataStr::from_str(left) < DataStr::from_str(right))
    }

    #[test]
    fn test_compare_2() {
        let left = "[[1],[2,3,4]]";
        let right = "[[1],4]";

        assert!(DataStr::from_str(left) < DataStr::from_str(right))
    }

    #[test]
    fn test_compare_3() {
        let left = "[9]";
        let right = "[[8,7,6]]";

        assert!(DataStr::from_str(left) > DataStr::from_str(right))
    }

    #[test]
    fn test_compare_4() {
        let left = "[[4,4],4,4]";
        let right = "[[4,4],4,4,4]";

        assert!(DataStr::from_str(left) < DataStr::from_str(right))
    }

    #[test]
    fn test_compare_5() {
        let left = "[7,7,7,7]";
        let right = "[7,7,7]";

        assert!(DataStr::from_str(left) > DataStr::from_str(right))
    }

    #[test]
    fn test_compare_6() {
        let left = "[]";
        let right = "[3]";

        assert!(DataStr::from_str(left) < DataStr::from_str(right))
    }

    #[test]
    fn test_compare_7() {
        let left = "[[[]]]";
        let right = "[[]]";

        assert!(DataStr::from_str(left) > DataStr::from_str(right))
    }

    #[test]
    fn test_compare_8() {
        let left = "[1,[2,[3,[4,[5,6,7]]]],8,9]";
        let right = "[1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert!(DataStr::from_str(left) > DataStr::from_str(right))
    }
}
