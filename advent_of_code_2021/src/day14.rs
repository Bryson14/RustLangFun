use crate::read_from_data_dir;
use regex::Regex;
use std::collections::HashMap;

/// # --- Day 14: Extended Polymerization ---
/// The incredible pressures at this depth are starting to put a strain on your submarine. The submarine has polymerization equipment that would produce suitable materials to reinforce the submarine, and the nearby volcanically-active caves should even have the necessary input elements in sufficient quantities.
///
/// The submarine manual contains instructions for finding the optimal polymer formula; specifically, it offers a polymer template and a list of pair insertion rules (your puzzle input). You just need to work out what polymer would result after repeating the pair insertion process a few times.
///
/// For example:
///
/// NNCB
///
/// CH -> B
/// HH -> N
/// CB -> H
/// NH -> C
/// HB -> C
/// HC -> B
/// HN -> C
/// NN -> C
/// BH -> H
/// NC -> B
/// NB -> B
/// BN -> B
/// BB -> N
/// BC -> B
/// CC -> N
/// CN -> C
/// The first line is the polymer template - this is the starting point of the process.
///
/// The following section defines the pair insertion rules. A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them. These insertions all happen simultaneously.
///
/// So, starting with the polymer template NNCB, the first step simultaneously considers all three pairs:
///
/// The first pair (NN) matches the rule NN -> C, so element C is inserted between the first N and the second N.
/// The second pair (NC) matches the rule NC -> B, so element B is inserted between the N and the C.
/// The third pair (CB) matches the rule CB -> H, so element H is inserted between the C and the B.
/// Note that these pairs overlap: the second element of one pair is the first element of the next pair. Also, because all pairs are considered simultaneously, inserted elements are not considered to be part of a pair until the next step.
///
/// After the first step of this process, the polymer becomes NCNBCHB.
///
/// Here are the results of a few steps using the above rules:
///
/// Template:     NNCB
/// After step 1: NCNBCHB
/// After step 2: NBCCNBBBCBHCB
/// After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
/// After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
/// This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times, H occurs 161 times, and N occurs 865 times; taking the quantity of the most common element (B, 1749) and subtracting the quantity of the least common element (H, 161) produces 1749 - 161 = 1588.
///
/// Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
pub fn part1() {
    let (pairs, template) = parse_data(read_from_data_dir("day14.txt").unwrap());
    let steps = 10;
    let final_polymer = grow_template(template, pairs, steps);
    let freq_diff = frequency_difference(final_polymer);
    println!(
        "Day14:1 The difference between the frequencies of charactes is {}.",
        freq_diff
    );
}

/// The main logic of the puzzle. Grows the template according to the rules
/// the main idea is that a index in a while loop crawls forward looking at the
/// current index and the one behind it. If it can insert a pair, it will and move the
/// index increments by 2, else if there is not pair to insert, the index increments by one.
fn grow_template(template: String, pair_rules: Vec<InsertionPair>, steps: u32) -> String {
    let mut polymer = template.clone();

    for _step in 0..steps {
        println!("Step: {}", _step);
        let mut idx: usize = 1;
        while idx < polymer.len() {
            let check_pair: String = String::from(&polymer[idx - 1..idx + 1]);

            if pair_rules.iter().any(|pair| pair.pair == check_pair) {
                pair_rules.iter().for_each(|pair| {
                    if pair.pair == check_pair {
                        polymer.insert(idx, pair.insert);
                        idx += 1;
                    }
                });
            }

            idx += 1;
        }
    }

    polymer
}

/// Grows the polymer according to the pair rules
/// Faster than the one above. Uses hashmaps to count the amonut of pairs, since each pair can be inserted
/// for example, "NNCB" is split into 3 pairs, "NN", "NC", and "CB" and stored in a hashmap with the values representing the quantity
/// this is then iterated over. "NN" turns into "NC" & "CN"
fn grow_template_hash_maps(template: String, pair_rules: Vec<InsertionPair>, steps: u32) -> String {
    "ok".into()
}

/// Looks at the frequncy of all the characters in the string.
/// subtracts the highest frequency from the lowest frequency to find the difference.
fn frequency_difference(s: String) -> i32 {
    let mut map: HashMap<char, i32> = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    map.values().max().unwrap() - map.values().min().unwrap()
}

fn parse_data(s: String) -> (Vec<InsertionPair>, String) {
    let mut pairs: Vec<InsertionPair> = Vec::new();
    let mut template = String::new();

    for line in s.lines() {
        if line.is_empty() {
            continue;
        } else if !line.contains("->") {
            template.push_str(line);
        } else if line.contains("->") {
            let items: Vec<String> = line.split("->").map(|s| String::from(s.trim())).collect();
            pairs.push(InsertionPair {
                pair: items[0].clone(),
                insert: items[1].chars().collect::<Vec<char>>()[0],
            });
        } else {
            unreachable!();
        }
    }

    (pairs, template)
}

#[derive(Debug)]
struct InsertionPair {
    pair: String,
    insert: char,
}

/// # --- Part Two ---
/// The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run more steps of the pair insertion process; a total of 40 steps should do it.
///
/// In the above example, the most common element is B (occurring 2192039569602 times) and the least common element is H (occurring 3849876073 times); subtracting these produces 2188189693529.
///
/// Apply 40 steps of pair insertion to the polymer template and find the most and least common elements in the result. What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
pub fn part2() {
    let (pairs, template) = parse_data(read_from_data_dir("day14.txt").unwrap());
    let steps = 40;
    let final_polymer = grow_template(template, pairs, steps);
    let freq_diff = frequency_difference(final_polymer);
    println!(
        "Day14:2 The difference between the frequencies of characters is at 40 steps {}.",
        freq_diff
    );
}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_difference() {
        let s = String::from("NNCB");
        // 2 N's - 1 C
        assert_eq!(frequency_difference(s), 1);
    }

    #[test]
    fn test_frequency_difference_2() {
        let s = String::from("NBCCNBBBCBHCB");
        // 6 B, 2 N, 4C, 1H -> 6-1=5
        assert_eq!(frequency_difference(s), 5);
    }

    #[test]
    fn test_grow_template() {
        let pairs = vec![
            InsertionPair {
                pair: "NN".into(),
                insert: 'C',
            },
            InsertionPair {
                pair: "NC".into(),
                insert: 'B',
            },
            InsertionPair {
                pair: "CB".into(),
                insert: 'H',
            },
        ];
        let polymer = grow_template("NNCB".into(), pairs, 1);
        assert_eq!(polymer, String::from("NCNBCHB"));
    }
}
