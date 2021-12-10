use crate::read_from_data_dir;

/// # --- Day 10: Syntax Scoring ---
/// You ask the submarine to determine the best route out of the deep-sea cave, but it only replies:
///
/// Syntax error in navigation subsystem on line: all of them
/// All of them?! The damage is worse than you thought. You bring up a copy of the navigation subsystem (your puzzle input).
///
/// The navigation subsystem syntax is made of several lines containing chunks. There are one or more chunks on each line, and chunks contain zero or more other chunks. Adjacent chunks are not separated by any delimiter; if one chunk stops, the next chunk (if any) can immediately start. Every chunk must open and close with one of four legal pairs of matching characters:
///
/// If a chunk opens with (, it must close with ).
/// If a chunk opens with [, it must close with ].
/// If a chunk opens with {, it must close with }.
/// If a chunk opens with <, it must close with >.
/// So, () is a legal chunk that contains no other chunks, as is []. More complex but valid chunks include ([]), {()()()}, <([{}])>, [<>({}){}[([])<>]], and even (((((((((()))))))))).
///
/// Some lines are incomplete, but others are corrupted. Find and discard the corrupted lines first.
///
/// A corrupted line is one where a chunk closes with the wrong character - that is, where the characters it opens and closes with do not form one of the four legal pairs listed above.
///
/// Examples of corrupted chunks include (], {()()()>, (((()))}, and <([]){()}[{}]). Such a chunk can appear anywhere within a line, and its presence causes the whole line to be considered corrupted.
///
/// For example, consider the following navigation subsystem:
/// ```text
/// [({(<(())[]>[[{[]{<()<>>
/// [(()[<>])]({[<{<<[]>>(
/// {([(<{}[<>[]}>{[]{[(<()>
/// (((({<>}<{<{<>}{[]{[]{}
/// [[<[([]))<([[{}[[()]]]
/// [{[{({}]{}}([{[{{{}}([]
/// {<[[]]>}<{[{[{[]{()[[[]
/// [<(<(<(<{}))><([]([]()
/// <{([([[(<>()){}]>(<<{{
/// <{([{{}}[<[[[<>{}]]]>[]]
/// ```
/// Some of the lines aren't corrupted, just incomplete; you can ignore these lines for now. The remaining five lines are corrupted:
///
/// ```text
/// {([(<{}[<>[]}>{[]{[(<()> - Expected ], but found } instead.
/// [[<[([]))<([[{}[[()]]] - Expected ], but found ) instead.
/// [{[{({}]{}}([{[{{{}}([] - Expected ), but found ] instead.
/// [<(<(<(<{}))><([]([]() - Expected >, but found ) instead.
/// <{([([[(<>()){}]>(<<{{ - Expected ], but found > instead.
/// ```
/// Stop at the first incorrect closing character on each corrupted line.
///
/// Did you know that syntax checkers actually have contests to see who can get the high score for syntax errors in a file? It's true! To calculate the syntax error score for a line, take the first illegal character on the line and look it up in the following table:
///
/// ): 3 points.
/// ]: 57 points.
/// }: 1197 points.
/// >: 25137 points.
/// In the above example, an illegal ) was found twice (2*3 = 6 points), an illegal ] was found once (57 points), an illegal } was found once (1197 points), and an illegal > was found once (25137 points). So, the total syntax error score for this file is 6+57+1197+25137 = 26397 points!
///
/// Find the first illegal character in each corrupted line of the navigation subsystem. What is the total syntax error score for those errors?
pub fn part1() {
    let data = read_from_data_dir("day10.txt").unwrap();
    let total_score: i32 = data
        .lines()
        .map(|line| find_illegal_char(String::from(line)))
        .filter(|op| op.is_some())
        .map(|op| get_score_illegal(op.unwrap()))
        .sum();
    println!("Day10:1 The total syntax error score is {}", total_score);
}

/// Returns the first closing bracket that is illegal and out of place
fn find_illegal_char(line: String) -> Option<char> {
    let mut brack_stack: Vec<char> = Vec::new();
    let mut ill_char: Option<char> = None;
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => brack_stack.push(c),
            ')' | ']' | '}' | '>' => {
                let popped = brack_stack.pop();
                if popped.is_none() {
                    // error there was no complementary closing brace
                    ill_char = Some(c);
                    break;
                }

                if get_matching_brace(c) != popped.unwrap() {
                    // non matching when popped
                    ill_char = Some(c);
                    break;
                }
            }
            _ => unreachable!(),
        };
    }

    ill_char
}

/// A helper function to return the pair of a bracket
/// i.e. the compliment of `)` is `(`
fn get_matching_brace(brace: char) -> char {
    return match brace {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        other @ _ => {
            panic!("Other sign: {} found!", other)
        }
    };
}

/// returns the illegal bracket score according to [Advent of Code 2021](https://adventofcode.com/2021/day/10)
fn get_score_illegal(illegal_bracket: char) -> i32 {
    return match illegal_bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        other @ _ => {
            panic!("Other sign: {} found!", other)
        }
    };
}

/// # --- Part Two ---
/// Now, discard the corrupted lines. The remaining lines are incomplete.
///
/// Incomplete lines don't have any incorrect characters - instead, they're missing some closing characters at the end of the line. To repair the navigation subsystem, you just need to figure out the sequence of closing characters that complete all open chunks in the line.
///
/// You can only use closing characters (), ], }, or >), and you must add them in the correct order so that only legal pairs are formed and all chunks end up closed.
///
/// In the example above, there are five incomplete lines:
///
/// ```text
/// [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
/// [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
/// (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
/// {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
/// <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
/// ```
/// Did you know that autocomplete tools also have contests? It's true! The score is determined by considering the completion string character-by-character. Start with a total score of 0. Then, for each character, multiply the total score by 5 and then increase the total score by the point value given for the character in the following table:
///
/// ): 1 point.
/// ]: 2 points.
/// }: 3 points.
/// >: 4 points.
/// So, the last completion string above - ])}> - would be scored as follows:
///
/// Start with a total score of 0.
/// Multiply the total score by 5 to get 0, then add the value of ] (2) to get a new total score of 2.
/// Multiply the total score by 5 to get 10, then add the value of ) (1) to get a new total score of 11.
/// Multiply the total score by 5 to get 55, then add the value of } (3) to get a new total score of 58.
/// Multiply the total score by 5 to get 290, then add the value of > (4) to get a new total score of 294.
/// The five lines' completion strings have total scores as follows:
///
/// }}]])})] - 288957 total points.
/// )}>]}) - 5566 total points.
/// }}>}>)))) - 1480781 total points.
/// ]]}}]}]}> - 995444 total points.
/// ])}> - 294 total points.
/// Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and then taking the middle score. (There will always be an odd number of scores to consider.) In this example, the middle score is 288957 because there are the same number of scores smaller and larger than it.
///
/// Find the completion string for each incomplete line, score the completion strings, and sort the scores. What is the middle score?
pub fn part2() {
    let data = read_from_data_dir("day10.txt").unwrap();
    // read the data in line by line,
    // remove the broken lines keeping only the incomplete,
    // get the missing sequences,
    // then score and get the middle score
    let mut scores: Vec<u64> = data
        .lines()
        .filter(|&line| find_illegal_char(String::from(line)).is_none())
        .map(|line| score_autocomplete_sequence(find_completing_sequence(line)))
        .collect::<Vec<u64>>();
    scores.sort();

    // assuming that there is an odd amount of scores
    println!("day10:2 The middle score is {}", scores[scores.len() / 2])
}

/// this assumes that the sequence is not broken as verified by [find_illegal_char]
/// this will only find the necessary character sequence to fix everything
/// might produce wrong output if the input sequence is illegal and not incomplete.
fn find_completing_sequence(line: &str) -> String {
    let mut brack_stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => brack_stack.push(c),
            ')' | ']' | '}' | '>' => {
                let popped = brack_stack.pop().unwrap();
                if get_matching_brace(c) != popped {
                    // non matching when popped
                    println!("Error matching {} and {}", popped, get_matching_brace(c));
                }
            }
            _ => unreachable!(),
        };
    }

    let fixing_seq: Vec<char> = brack_stack
        .iter()
        .rev()
        .map(|&c| get_matching_brace(c))
        .collect::<Vec<char>>();
    let mut complete_seq = String::new();
    fixing_seq.iter().for_each(|&c| complete_seq.push(c));

    complete_seq
}

/// Takes a sequence of closing brackets that solve the issue like "])}>" and applies the rules line by line
/// 1. takes the total score and multiplies by 5
/// 2. add the score for the next char and adds that to the total score (']' = 2)
/// 3. repeat
fn score_autocomplete_sequence(sequence: String) -> u64 {
    return sequence
        .chars()
        .map(|c| get_score_auto_complete(c))
        .fold(0u64, |sum, i| (sum * 5) + i as u64);
}

/// returns the autocomplete score according to [Advent of Code 2021](https://adventofcode.com/2021/day/10)
fn get_score_auto_complete(missing_bracket: char) -> i32 {
    return match missing_bracket {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        other @ _ => {
            panic!("Other sign: {} found!", other)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_illegal_char() {
        assert_eq!(find_illegal_char(String::from("[[()]]")), None);
    }

    #[test]
    fn test_find_illegal_char_2() {
        // this line is incomplete and shouldn't through an error
        assert_eq!(find_illegal_char(String::from("[[()]]{")), None);
    }

    #[test]
    fn test_find_illegal_char_3() {
        assert_eq!(
            find_illegal_char(String::from("{([(<{}[<>[]}>{[]{[(<()>")),
            Some('}')
        );
    }

    #[test]
    fn test_find_illegal_char_4() {
        assert_eq!(
            find_illegal_char(String::from("[({(<(())[]>[[{[]{<()<>>")),
            None
        );
    }

    #[test]
    fn test_score_autocomplete_sequence() {
        assert_eq!(score_autocomplete_sequence(String::from("])}>")), 294);
    }

    #[test]
    fn test_score_autocomplete_sequence_2() {
        assert_eq!(
            score_autocomplete_sequence(String::from("}}]])})]")),
            288957
        );
    }

    #[test]
    fn test_find_completing_sequence() {
        assert_eq!(
            find_completing_sequence("[({(<(())[]>[[{[]{<()<>>"),
            String::from("}}]])})]")
        );
    }
}
