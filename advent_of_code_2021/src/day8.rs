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
    let total_of_outputs: u32 = data
        .lines()
        .map(|line| solve_output(split_data(line)))
        .sum();

    println!(
        "Day8:2. The total sum of the outputs is {}",
        total_of_outputs
    );
}

/// Takes the entire line, such as `gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce`
/// and solves the message, then returns the last four numbers as u32
/// the overall idea, is that is goes over all the letters, knowing that some will line up with certain outputs.todo!
/// The truth_map is a 7x7 grid that represents the mapping between each letter and the actual output
///  2222
/// 1    3
/// 1    3
///  7777
/// 6    4
/// 6    4
///  5555
/// for example, passing `ae` must be the number 1, so now we know `sae` must map to `3` and `4`
/// assumes the last 4 tokens are the output tokens, or the ones after the `|`
fn solve_output<'a>(tokens: Vec<&'a str>) -> u32 {
    let mut truth_map = vec![vec![true; 7]; 7];
    // solve for the map
    tokens
        .iter()
        .for_each(|token| deduce_map(&mut truth_map, token));

    println!("truth map\n: {:?}", truth_map);

    // convert tokens
    let converted = tokens
        .iter()
        .rev()
        .take(4)
        .map(|&token| convert_token_to_num(&truth_map, token))
        .collect::<Vec<u32>>();

    // put together
    converted[0] * 1000 + converted[1] * 100 + converted[2] * 10 + converted[3]
}

/// helper function to turn "abd" to vec[0,1,3]
fn chars_to_idx<'a>(token: &'a str) -> Vec<usize> {
    token
        .chars()
        .map(|c| match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => unreachable!(),
        })
        .collect()
}

/// the map is set up as such:
/// . 1 2 3 4 5 6 7
/// a . . . . . . .
/// b . . . . . . .
/// c . . . . . . .
/// d . . . . . . .
/// e . . . . . . .
/// f . . . . . . .
/// g . . . . . . .
fn deduce_map<'a>(truth_map: &mut Vec<Vec<bool>>, token: &'a str) {
    let l = token.len();
    let row_idxs = chars_to_idx(token);
    if l == 2 {
        // 1, wires 3,4
        change_truth_map(truth_map, row_idxs, vec![2, 3]);
    } else if l == 3 {
        // 7, wires 2,3,4
        change_truth_map(truth_map, row_idxs, vec![1, 2, 3]);
    } else if l == 4 {
        // 4, wires 1,3,4,7
        change_truth_map(truth_map, row_idxs, vec![0, 2, 3, 6]);
    } else if l == 5 {
        // 2, wires 2,3,5,6,7
        // 3, wires 2,3,4,5,7
        // 5, wires 1,2,4,5,7
    } else if l == 6 {
        // 6, wires 1,2,4,5,6,7
        // 9, wires 1,2,3,4,5,7
        // 0, wires 1,2,3,4,5,6
    } else if l == 7 {
        // 8, all wires
    } else {
        unreachable!();
    }
}

/// changes the entire row that is not in `true_cols` to false
/// so `row_idxs: vec[0,1], col: vec[6,7]` would change all of row 0 and 1 to `false` except columns 6 and 7  
fn change_truth_map(truth_map: &mut Vec<Vec<bool>>, row_idxs: Vec<usize>, true_cols: Vec<usize>) {
    for row in row_idxs.iter() {
        for col in 0..7 {
            if !true_cols.contains(&col) {
                truth_map[*row][col] = false;
            }
        }
    }
}

/// now taking the filtered truth map, and the token, it finds what each section of the display is lit up, and converts that to a number
fn convert_token_to_num<'a>(truth_map: &[Vec<bool>], token: &'a str) -> u32 {
    let row_idxs = chars_to_idx(token);
    let mut lit_sections: Vec<usize> = row_idxs
        .iter()
        .map(|row| truth_map[*row].iter().position(|boolean| *boolean).unwrap())
        .collect();

    lit_sections.sort();

    if lit_sections == vec![1, 2, 3, 4, 5, 6] {
        return 0;
    } else if lit_sections == vec![3, 4] {
        return 1;
    } else if lit_sections == vec![2, 3, 5, 6, 7] {
        return 2;
    } else if lit_sections == vec![2, 3, 4, 5, 7] {
        return 3;
    } else if lit_sections == vec![1, 3, 4, 7] {
        return 4;
    } else if lit_sections == vec![1, 2, 4, 5, 7] {
        return 5;
    } else if lit_sections == vec![1, 2, 4, 5, 6, 7] {
        return 6;
    } else if lit_sections == vec![2, 3, 4] {
        return 7;
    } else if lit_sections == vec![1, 2, 3, 4, 5, 6, 7] {
        return 8;
    } else if lit_sections == vec![1, 2, 3, 4, 5, 7] {
        return 9;
    } else {
        println!("Wires were {:?}", lit_sections);
        unreachable!();
    }
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
    false
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_chars_to_idx() {
        assert_eq!(chars_to_idx("abde"), vec![0, 1, 3, 4]);
    }

    #[test]
    fn test_solve_output() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let splits = split_data(input);
        let output = solve_output(splits);
        assert_eq!(output, 5353);
    }
}
