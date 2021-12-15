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
/// this is super slow and unsuable after steps > 17 because of memory usage
fn grow_template(template: String, pair_rules: Vec<InsertionPair>, steps: u32) -> String {
    let mut polymer = template.clone();

    for _step in 0..steps {
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

/// takes a starting polymer like "NNCB" and splits it into the pairs "NN", "NC", "CB" and puts them in a hashmap to count their frequency in the polymer
fn polymer_to_hashmap(polymer: String) -> HashMap<String, i64> {
    let mut pairs_table: HashMap<String, i64> = HashMap::new();
    let mut idx = 1;
    while idx < polymer.len() {
        let check_pair: String = String::from(&polymer[idx - 1..idx + 1]);
        *pairs_table.entry(String::from(check_pair)).or_insert(0) += 1;
        idx += 1;
    }
    pairs_table
}

/// Grows the polymer according to the pair rules
/// Faster than the one above. Uses hashmaps to count the amonut of pairs, since each pair can be inserted
/// for example, "NNCB" is split into 3 pairs, "NN", "NC", and "CB" and stored in a hashmap with the values representing the quantity
/// this is then iterated over. "NN" turns into "NC" & "CN" and added into the hashmap. ("NN" is taken out).
/// when counting the frequency at the end, only the first character in every pair is considered since every character is counted twice in this system
/// with exception of the very last character of `template` which is not counted twice and will need to be added into the frequqncy.
fn grow_template_hash_maps(
    template: String,
    pair_rules: Vec<InsertionPair>,
    steps: u32,
) -> HashMap<String, i64> {
    let mut pairs_table = polymer_to_hashmap(template);
    // println!("map: {:?}", pairs_table);

    for _step in 0..steps {
        let mut pairs_table_next_step: HashMap<String, i64> = HashMap::new();
        let copy_of_keys: Vec<String> = pairs_table.keys().map(|s| String::from(s)).collect();
        for key in copy_of_keys {
            if pair_rules.iter().any(|pair| pair.pair == key) {
                pair_rules.iter().for_each(|pair| {
                    if pair.pair == key {
                        // getting the two new pairs from the insertion
                        // i.e. "NC->B" will be "NB" and "BC"

                        let amount: i64 = *pairs_table.get(&key).unwrap();

                        let mut new_pair_1 = String::from(&pair.pair[0..1]);
                        new_pair_1.push(pair.insert);
                        let mut new_pair_2 = String::from(pair.insert);
                        new_pair_2.push_str(&pair.pair[1..2]);

                        // println!("new pair: {}, new pair 2: {}", new_pair_1, new_pair_2);

                        *pairs_table_next_step.entry(new_pair_1).or_insert(0) += amount;
                        *pairs_table_next_step.entry(new_pair_2).or_insert(0) += amount;
                    }
                });
            } else {
                // if there is no rule for that pair, then just pass it onto the next iteration
                *pairs_table_next_step.entry(key.clone()).or_insert(0) +=
                    pairs_table.get(&key).unwrap();
            }
        }
        pairs_table = pairs_table_next_step;
        // println!("map: {:?}", pairs_table);
    }
    pairs_table
}

/// Looks at the frequncy of all the characters in the string.
/// subtracts the highest frequency from the lowest frequency to find the difference.
fn frequency_difference(s: String) -> i64 {
    let mut map: HashMap<char, i64> = HashMap::new();
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
    let steps = 1;
    let final_polymer = grow_template_hash_maps(template, pairs, steps);
    // count the hashmap
    let freq_diff = frequency_difference("ok".into());
    println!(
        "Day14:2 The difference between the frequencies of characters is at 40 steps {}.",
        freq_diff
    );
}

/// similar to [frequency_difference], it find the difference between the most common and least common character.
/// However, this does it with a hashmap, knowing that the second character in each pair is ignored
fn frequency_difference_hashmap(map: HashMap<String, i64>, last_char: char) -> i64 {
    let mut frequencies: HashMap<char, i64> = HashMap::new();
    // getting the frequcnies of the first letter in each pair
    map.keys().for_each(|s| {
        let first_letter: char = s.chars().collect::<Vec<char>>()[0];
        *frequencies.entry(first_letter).or_insert(0) += map.get(s).unwrap();
    });

    *frequencies.entry(last_char).or_insert(0) += 1;
    frequencies.values().max().unwrap() - map.values().min().unwrap()
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

    #[test]
    fn test_grow_template_hash() {
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
        let polymer = grow_template_hash_maps("NNCB".into(), pairs, 1);

        assert_eq!(frequency_difference_hashmap(polymer, 'B'), 1);
    }

    #[test]
    fn test_with_test_data() {
        let (pairs, template) = parse_data(read_from_data_dir("day14_test.txt").unwrap());
        let final_character: char = template.chars().rev().take(1).collect::<Vec<char>>()[0];
        let steps = 10;
        let final_polymer = grow_template_hash_maps(template, pairs, steps);
        let freq_diff = frequency_difference_hashmap(final_polymer, final_character);
        assert_eq!(freq_diff, 1588);
    }

    fn test_with_test_data_3() {
        let (pairs, template) = parse_data(read_from_data_dir("day14_test.txt").unwrap());
        let final_character: char = template.chars().rev().take(1).collect::<Vec<char>>()[0];
        let steps = 10;
        let final_polymer = grow_template_hash_maps(template, pairs, steps);
        let freq_diff = frequency_difference_hashmap(final_polymer.clone(), final_character);
        assert_eq!(
            final_polymer,
            polymer_to_hashmap("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".into())
        );
        assert_eq!(freq_diff, 1588);
    }

    #[test]
    fn test_with_test_data_2() {
        let (pairs, template) = parse_data(read_from_data_dir("day14_test.txt").unwrap());
        let final_character: char = template.chars().rev().take(1).collect::<Vec<char>>()[0];
        let steps = 40;
        let final_polymer = grow_template_hash_maps(template, pairs, steps);
        let freq_diff = frequency_difference_hashmap(final_polymer, final_character);
        assert_eq!(freq_diff, 2188189693529);
    }
}
