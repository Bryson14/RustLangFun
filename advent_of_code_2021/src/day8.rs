use crate::read_from_data_dir;

/// # --- Day 8: Seven Segment Search ---
/// You barely reach the safety of the cave when the whale smashes into the cave mouth, collapsing it. Sensors indicate another exit to this cave at a much greater depth, so you have no choice but to press on.
///
/// As your submarine slowly makes its way through the cave system, you notice that the four-digit seven-segment displays in your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a lot of trouble without them, so you'd better figure out what's wrong.
///
/// Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:
/// ```text
///   0:      1:      2:      3:      4:
///  aaaa    ....    aaaa    aaaa    ....
/// b    c  .    c  .    c  .    c  b    c
/// b    c  .    c  .    c  .    c  b    c
///  ....    ....    dddd    dddd    dddd
/// e    f  .    f  e    .  .    f  .    f
/// e    f  .    f  e    .  .    f  .    f
///  gggg    ....    gggg    gggg    ....
///
///   5:      6:      7:      8:      9:
///  aaaa    aaaa    aaaa    aaaa    aaaa
/// b    .  b    .  .    c  b    c  b    c
/// b    .  b    .  .    c  b    c  b    c
///  dddd    dddd    ....    dddd    dddd
/// .    f  e    f  .    f  e    f  .    f
/// .    f  e    f  .    f  e    f  .    f
///  gggg    gggg    ....    gggg    gggg
/// ```
/// So, to render a 1, only segments c and f would be turned on; the rest would be off. To render a 7, only segments a, c, and f would be turned on.
///
/// The problem is that the signals which control the segments have been mixed up on each display. The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly. Worse, the wire/segment connections are mixed up separately for each four-digit display! (All of the digits within a display use the same connections, though.)
///
/// So, you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on. With just that information, you still can't tell which wire (b/g) goes to which segment (c/f). For that, you'll need to collect more information.
///
/// For each display, you watch the changing signals for a while, make a note of all ten unique signal patterns you see, and then write down a single four digit output value (your puzzle input). Using the signal patterns, you should be able to work out which pattern corresponds to which digit.
///
/// For example, here is what you might see in a single entry in your notes:
/// ```text
/// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
/// ```
/// (The entry is wrapped here to two lines so it fits; in your notes, it will all be on a single line.)
///
/// Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value. Within an entry, the same wire/segment connections are used (but you don't know what the connections actually are). The unique signal patterns correspond to the ten different ways the submarine tries to render a digit using the current wire/segment connections. Because 7 is the only digit that uses three segments, dab in the above example means that to render a 7, signal lines d, a, and b are on. Because 4 is the only digit that uses four segments, eafb means that to render a 4, signal lines e, a, f, and b are on.
///
/// Using this information, you should be able to work out which combination of signal wires corresponds to each of the ten digits. Then, you can decode the four digit output value. Unfortunately, in the above example, all of the digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and are more difficult to deduce.
///
/// For now, focus on the easy digits. Consider this larger example:
/// ```text
/// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
/// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
/// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
/// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
/// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
/// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
/// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
/// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
/// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
/// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
/// ```
/// Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should be able to tell which combinations of signals correspond to those digits. Counting only digits in the output values (the part after | on each line), in the above example, there are 26 instances of digits that use a unique number of segments (highlighted above).
///
/// In the output values, how many times do digits 1, 4, 7, or 8 appear? (that is, after the `|`?)
pub fn part1() {
    let data = read_from_data_dir("day8.txt").unwrap();
    let mut count = 0;
    for line in data.lines() {
        let v = split_data(line);
        count += v
            .iter()
            .rev()
            .take(4)
            .filter(|token| matches!(token.len(), 2 | 4 | 3 | 7))
            .count();
    }
    println!(
        "Day8:1 Number of 1, 4, 8, & 7s that appear in the output data is: {}",
        count
    );
}

/// separates the data input
/// i.e. debca bgc | ceafbd will turn into Vec['debca', 'bgc', 'ceafbd]
fn split_data<'a>(s: &'a str) -> Vec<&'a str> {
    let mut words: Vec<&'a str> = Vec::with_capacity(15);
    s.split(" | ").for_each(|chunk| {
        chunk
            .split_ascii_whitespace()
            .for_each(|word| words.push(word))
    });
    words
}

/// # --- Part Two ---
/// Through a little deduction, you should now be able to determine the remaining digits. Consider again the first example above:
///
/// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
/// cdfeb fcadb cdfeb cdbaf
/// After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:
///
///  dddd
/// e    a
/// e    a
///  ffff
/// g    b
/// g    b
///  cccc
/// So, the unique signal patterns would correspond to the following digits:
///
/// acedgfb: 8
/// cdfbe: 5
/// gcdfa: 2
/// fbcad: 3
/// dab: 7
/// cefabd: 9
/// cdfgeb: 6
/// eafb: 4
/// cagedb: 0
/// ab: 1
/// Then, the four digits of the output value can be decoded:
///
/// cdfeb: 5
/// fcadb: 3
/// cdfeb: 5
/// cdbaf: 3
/// Therefore, the output value for this entry is 5353.
///
/// Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:
///
/// fdgacbe cefdb cefbgd gcbe: 8394
/// fcgedb cgb dgebacf gc: 9781
/// cg cg fdcagb cbg: 1197
/// efabcd cedba gadfec cb: 9361
/// gecf egdcabf bgf bfgea: 4873
/// gebdcfa ecba ca fadegcb: 8418
/// cefg dcbef fcge gbcadfe: 4548
/// ed bcgafe cdgba cbgef: 1625
/// gbdfcae bgc cg cgb: 8717
/// fgae cfgab fg bagce: 4315
/// Adding all of the output values in this larger example produces 61229.
///
/// For each entry, determine all of the wire/segment connections and decode the four-digit output values. What do you get if you add up all of the output values?
///
/// The idea with this is to try permutations, but not all 4000 that could be generated. By using a backtracking algorithm,
/// it will attempt to solve different connections. The solution grid will be 7x7 representing the 7 wires going into the
/// display vs the 7 segments that ar getting lit up. At first, all the grid is set to `true`. As the backtracking algorithm
/// works, it will take what it knows about certain characters `1`, `7`, `4`, `6` to eliminate possibilities and leave only
/// one `true` in each column of the solution map. This will represent the connection between the input and output.
pub fn part2() {
    let data = read_from_data_dir("day8.txt").unwrap();
    let total_of_outputs: u32 = data.lines().map(solve_output).sum();

    println!(
        "Day8:2. The total sum of the outputs is {}",
        total_of_outputs
    );
}

/// Takes the entire line, such as `gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`
/// and solves the message, then returns the last four numbers as u32
/// the overall idea, is that is goes over all the letters, knowing that some will line up with certain outputs.todo!
/// The truth_map is a 7x7 grid that represents the mapping between each letter and the actual output
///
/// for example, passing `ae` must be the number 1, so now we know `sae` must map to `3` and `4`
/// assumes the last 4 tokens are the output tokens, or the ones after the `|`
fn solve_output(data_line: &str) -> u32 {
    let mut zero_2_ten = Vec::new();
    let mut four_digit_ouput = Vec::new();
    let mut pip_found = false;
    for token in data_line.split(' ') {
        if token == "|" {
            pip_found = true;
        } else if pip_found {
            four_digit_ouput.push(token);
        } else {
            zero_2_ten.push(token);
        }
    }

    let solved_map = deduce_map(zero_2_ten);
    let output_sorted: Vec<String> = four_digit_ouput
        .iter()
        .map(|tok| tok.chars().sorted().collect::<String>())
        .collect();

    println!("digits: {:?}", output_sorted);
    let mut output = 0;
    let mut place_value = 4;
    for digit in output_sorted.iter() {
        place_value -= 1;
        output += solved_map.get(digit).unwrap() * 10i32.pow(place_value);
    }
    output as u32
}

use itertools::Itertools;
use std::collections::HashMap;

/// returns a hashmap that correlates a sorted string to the char
/// i.e. 'ad' -> 1, 'abd' -> 7
fn deduce_map(zero_2_ten: Vec<&str>) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    // sorting all
    let tokens: Vec<String> = zero_2_ten
        .iter()
        .map(|tok| tok.chars().sorted().collect::<String>())
        .collect();

    // inserting 1,4,7,8 because those are known by length of str
    tokens
        .iter()
        .filter(|tok| tok.len() == 2 || tok.len() == 3 || tok.len() == 4 || tok.len() == 7)
        .for_each(|tok| {
            let sorted_token = tok.chars().sorted().collect::<String>();
            map.insert(sorted_token, get_number(tok.len()));
        });
    let four = get_key_from_value(&map, &4);
    let seven = get_key_from_value(&map, &7);

    assert_eq!(map.len(), 4);

    // 9 is len=6 and contains 4
    let nine: &String = tokens
        .iter()
        .filter(|&tok| tok.len() == 6 && contains_chars(tok, &four))
        .collect::<Vec<&String>>()[0];
    map.insert(nine.clone(), 9);

    assert_eq!(map.len(), 5);

    // 3 is len=5 and contains 7
    let three: &String = tokens
        .iter()
        .filter(|&tok| tok.len() == 5 && contains_chars(tok, &seven))
        .collect::<Vec<&String>>()[0];
    map.insert(three.clone(), 3);

    assert_eq!(map.len(), 6);

    // 0 is len=6, contains 7 but is not 9
    let zero: &String = tokens
        .iter()
        .filter(|&tok| tok.len() == 6 && contains_chars(tok, &seven) && tok != nine)
        .collect::<Vec<&String>>()[0];
    map.insert(zero.clone(), 0);

    assert_eq!(map.len(), 7);

    // 6 is len=6, not 0 or 9
    let six: &String = tokens
        .iter()
        .filter(|tok| tok.len() == 6 && !map.contains_key(&*tok.to_string()))
        .collect::<Vec<&String>>()[0];
    map.insert(six.clone(), 6);

    assert_eq!(map.len(), 8);

    // 5 is len=5, contained by 9
    let five: &String = tokens
        .iter()
        .filter(|tok| {
            tok.len() == 5 && contains_chars(nine, tok) && !map.contains_key(&*tok.to_string())
        })
        .collect::<Vec<&String>>()[0];
    map.insert(five.clone(), 5);

    assert_eq!(map.len(), 9);

    // 2 is len=5, not 3 or 5
    let two: &String = tokens
        .iter()
        .filter(|tok| tok.len() == 5 && !map.contains_key(&*tok.to_string()))
        .collect::<Vec<&String>>()[0];
    map.insert(two.clone(), 2);

    assert_eq!(map.len(), 10);

    println!("map: {:?}", map);

    map
}

fn get_number(str_len: usize) -> i32 {
    match str_len {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => unreachable!(),
    }
}

/// checks if one string contains all the other character of another string
fn contains_chars(container: &str, contained: &str) -> bool {
    contained.chars().all(|c| container.contains(c))
}

/// gets the string key by looking at the unique values of the hashmap.
/// Values and keys all must be unique
fn get_key_from_value(map: &HashMap<String, i32>, val: &i32) -> String {
    for (key, value) in map {
        if value == val {
            return key.clone();
        }
    }
    String::new()
}

// 0: 6 segment
// 1: 2 segment
// 2: 5 segment
// 3: 5 segment
// 4: 4 segment
// 5: 5 segment
// 6: 6 segment
// 7: 3 segment
// 8: 7 segment
// 9: 6 segment

pub fn is_complete() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_split_data() {
        let s = "cg fadegbc ecfadb acdbeg abgfe dcegfb gcad bceag debca bgc | ceafbd gfedcb cabedf dbace";
        assert_eq!(
            split_data(s),
            vec![
                "cg", "fadegbc", "ecfadb", "acdbeg", "abgfe", "dcegfb", "gcad", "bceag", "debca",
                "bgc", "ceafbd", "gfedcb", "cabedf", "dbace"
            ]
        );
    }

    #[test]
    fn test_solve_output() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let output = solve_output(input);
        assert_eq!(output, 5353);
    }

    #[test]
    fn test_deduce_map() {}

    #[test]
    fn test_get_key_by_value() {
        let mut map = HashMap::new();
        map.insert("abcd".into(), 4);
        map.insert("abcdefg".into(), 8);

        assert_eq!(get_key_from_value(&map, &8), String::from("abcdefg"));
    }

    #[test]
    fn test_contains_chars() {
        let a = "abcd";
        let b = "ac";
        assert_eq!(contains_chars(a, b), true);
    }
}
