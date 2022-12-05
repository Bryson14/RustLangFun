use crate::utils::read_data;
use regex::Regex;

const FILE: &str = "day5.txt";
const DAY: &str = "{{ DAY 5 }}";

/// --- Day 5: Supply Stacks ---
pub fn part1() {
    let data = read_data(FILE);
    let mut crane = parse_data(data);
    for i in 0..crane.moves.len() {
        crane.execute_move_9000(i);
    }
    // get the last element of each stack or the top
    let tops = crane
        .crates
        .iter()
        .filter_map(|stack| stack.last())
        .map(|c| c.to_owned())
        .collect::<Vec<char>>();

    let ans = String::from_iter(tops);
    println!("{DAY} The top crates are {ans}");
}

pub fn part2() {
    let data = read_data(FILE);
    let mut crane = parse_data(data);
    for i in 0..crane.moves.len() {
        crane.execute_move_9001(i);
    }
    // get the last element of each stack or the top
    let tops = crane
        .crates
        .iter()
        .filter_map(|stack| stack.last())
        .map(|c| c.to_owned())
        .collect::<Vec<char>>();

    let ans = String::from_iter(tops);
    println!("{DAY} The top crates are {ans}");
}

/// Move is the instruction to move a crate. This is 1-based indexing to match the input data
struct Move {
    qty: i32,
    dest: i32,
    target: i32,
}

struct CraneData {
    moves: Vec<Move>,
    crates: Vec<Vec<char>>,
}

impl CraneData {
    // the crates are indexed into with location - 1 since the moves are 1-based indexing
    fn execute_move_9000(&mut self, move_idx: usize) {
        let m: &Move = self.moves.get(move_idx).unwrap();
        for _ in 0..m.qty {
            let c = self.crates[(m.dest - 1) as usize]
                .pop()
                .unwrap_or_else(|| panic!("No crate found in stack {}", m.dest));
            self.crates[(m.target - 1) as usize].push(c);
        }
    }

    /// The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
    /// The CrateMover 9001 is notable for many new and exciting features: air conditioning,
    /// leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
    fn execute_move_9001(&mut self, move_idx: usize) {
        let m: &Move = self.moves.get(move_idx).unwrap();
        for i in 0..m.qty {
            let target_len = self.crates[(m.target - 1) as usize].len();
            let c = &mut self.crates[(m.dest - 1) as usize]
                .pop()
                .unwrap_or_else(|| panic!("No crate found in stack {}", m.dest));
            let _ = &mut self.crates[(m.target - 1) as usize].insert(target_len - (i as usize), *c);
        }
    }
}

fn parse_data(data: String) -> CraneData {
    let mut moves: Vec<Move> = Vec::new();
    let mut crates: Vec<Vec<char>> = Vec::new();
    let re_moves = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    let mut max_col = 9;
    for line in data.lines() {
        if line.contains("move") {
            let caps = re_moves.captures(line).unwrap();

            let qty = caps
                .get(1)
                .expect("no qty number found")
                .as_str()
                .parse::<i32>()
                .expect("Cannot parse qty");
            let dest = caps
                .get(2)
                .expect("no dest number found")
                .as_str()
                .parse::<i32>()
                .expect("Cannot parse dest");
            let target = caps
                .get(3)
                .expect("no target number found")
                .as_str()
                .parse::<i32>()
                .expect("Cannot parse target");

            moves.push(Move { qty, dest, target });
        } else if line.contains('1') {
            let nine = '9'.to_digit(10).unwrap();
            let zero = '0'.to_digit(10).unwrap();
            max_col = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .filter(|&d| d <= nine && d >= zero)
                .max()
                .unwrap();
            println!("Max column is {max_col}");
        } else if line.contains('[') {
            // know the index of the line where the data is
            // first col = 2, second = 6, 10, 14, 18, 22, 26, 30, 34  (1 based index)
            // so a 'H' in 6th index in the line will be put in the second stack
            if crates.len() != max_col as usize {
                while crates.len() < max_col as usize {
                    crates.push(Vec::new());
                }
            }
            for (i, val) in line.chars().enumerate() {
                match val {
                    '[' | ']' | '\n' | ' ' => {}
                    'A'..='Z' => {
                        if let Some(stack) = crates.get_mut(i / 4) {
                            stack.insert(0, val)
                        } else {
                            unreachable!();
                        }
                    }
                    _ => unreachable!(),
                }
            }
        } else {
            continue;
        }
    }
    CraneData { moves, crates }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data() {}

    #[test]
    fn test_execute_move_9000() {
        let mut data = CraneData {
            moves: vec![Move {
                qty: 1,
                dest: 2,
                target: 1,
            }],
            crates: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        data.execute_move_9000(0);
        println!("crate: {:?}", data.crates);
        assert!(data.crates == vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']])
    }

    #[test]
    fn test_execute_move_9001() {
        let mut data = CraneData {
            moves: vec![Move {
                qty: 2,
                dest: 2,
                target: 1,
            }],
            crates: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        };
        data.execute_move_9001(0);
        println!("crate: {:?}", data.crates);
        assert!(data.crates == vec![vec!['Z', 'N', 'C', 'D'], vec!['M'], vec!['P']])
    }
}
