use crate::utils::read_data;

const FILE: &str = "day6.txt";
const DAY: &str = "{{ DAY 6 }}";

/// --- Day 6: Tuning Trouble ---
/// To fix the communication system, you need to add a subroutine to the device that detects
/// a start-of-packet marker in the datastream. In the protocol being used by the Elves,
/// the start of a packet is indicated by a sequence of four characters that are all different.
/// Here are a few more examples:
///
/// bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
/// nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
/// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
/// zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
/// How many characters need to be processed before the first start-of-packet marker is detected?
pub fn part1() {
    let data = read_data(FILE);
    let ans = hashset_iter_check(data.as_str(), 4);
    println!("{DAY}-1 Packet starts after {ans} chars");
    benchmark(&hashset_iter_check, 1000, 4, data.as_str());
}

pub fn part2() {
    let data = read_data(FILE);
    let ans = hashset_iter_check(data.as_str(), 14);
    println!("{DAY}-2 Packet starts after {ans} chars");
    benchmark(&hashset_iter_check, 1000, 14, data.as_str());
}

fn benchmark(func: &dyn Fn(&str, usize) -> usize, iterations: u32, buffer: usize, data: &str) {
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    for _ in 0..iterations {
        let _ = func(data, buffer);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.3?}", elapsed);
    println!("Time per call: {:.4?}", elapsed / iterations)
}

/// benchmarks
/// (4 chars start buffer) debug: 3.5ms / iter. Release: 234µs / iter (14.9x speedup)
/// (14 chars start buffer) debug: 7.966ms / iter. Release: 727µs / iter (11x speedup)
/// 1.02 - 3.8x better than the bashset function
fn itertools_iter_buffer_check(s: &str, no_repeat_len: usize) -> usize {
    use itertools::Itertools;

    let s = s.as_bytes();
    let marker_idx = s
        .windows(no_repeat_len)
        .position(|c| c.iter().all_unique())
        .expect("Should find a buffer starter packet");

    marker_idx + no_repeat_len
}

/// # benchmarks
/// ## Making a new hashset in everyloop
/// (4 chars start buffer) debug: 4.1542ms / iter. Release: 239µs / iter (17x speedup)
/// (14 chars start buffer) debug: 22.77ms / iter. Release: 2.79ms / iter (8x speedup)
/// ## Making on hashset and clearing it every round.
/// (4 chars start buffer) debug: 2.78ms / iter. Release: 104µs / iter (17x speedup)
/// (14 chars start buffer) debug: 12.79ms / iter. Release: 638µs / iter (8x speedup)
/// Speed up form previous hashset implementation: ~2x debug or 4x speedup
fn hashset_iter_check(s: &str, no_repeat_len: usize) -> usize {
    use std::collections::HashSet;
    let mut marker = no_repeat_len - 1;
    // creating a larger capacity than ever filled to reduce chances of collisions in the hashset
    let mut set: HashSet<&u8> = HashSet::with_capacity(no_repeat_len * 2);
    // add first n elements to the hash
    s[0..no_repeat_len].as_bytes().iter().for_each(|c| {
        let _ = set.insert(c);
    });
    for i in no_repeat_len..s.len() {
        // if the hashset has all elements and there were no repeated characters,
        // then all elements were
        if set.len() < no_repeat_len {
            marker += 1;
            set.clear();
            s[i - no_repeat_len..i].as_bytes().iter().for_each(|c| {
                let _ = set.insert(c);
            });
        } else {
            return marker;
        }
    }
    marker
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_check() {
        let no_repeat_len = 4;
        let buffer = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, hashset_iter_check(buffer, no_repeat_len));

        let buffer = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, itertools_iter_buffer_check(buffer, no_repeat_len));

        let buffer = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, itertools_iter_buffer_check(buffer, no_repeat_len));

        let buffer = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, itertools_iter_buffer_check(buffer, no_repeat_len));

        let buffer = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, itertools_iter_buffer_check(buffer, no_repeat_len));
    }

    #[test]
    fn test_iter_check_longer() {
        let no_repeat_len = 14;
        let buffer = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, hashset_iter_check(buffer, no_repeat_len));

        let buffer = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, hashset_iter_check(buffer, no_repeat_len));

        let buffer = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, hashset_iter_check(buffer, no_repeat_len));

        let buffer = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, hashset_iter_check(buffer, no_repeat_len));

        let buffer = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, hashset_iter_check(buffer, no_repeat_len));
    }
}
