// src/day01/mod.rs

use std::collections::HashMap;

use crate::utils;
mod part1;
mod part2;

pub fn run(part: Option<u32>, test_mode: bool) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data: String;
    let mut example_answer: String = String::from("None");
    const DAY: &str = "08";

    if test_mode {
        let part_str = match part {
            Some(1) => "1",
            Some(2) => "2",
            _ => "1",
        };
        let (example_input, ans) = utils::get_example(DAY, part_str);
        input_data = example_input;
        example_answer = ans;
    } else {
        input_data = utils::get_input(DAY);
    }

    match part {
        Some(1) => {
            println!("Running Part 1");
            part1::solve(&input_data);
        }
        Some(2) => {
            println!("Running Part 2");
            part2::solve(&input_data);
        }
        _ => {
            // Default behavior: run both parts
            println!("Running Part 1");
            part1::solve(&input_data);

            println!("Running Part 2");
            part2::solve(&input_data);
        }
    }

    if test_mode {
        println!("Example Answer: {}", example_answer);
    }
}

#[derive(Debug, PartialEq)]
pub struct TwoPathNetwork<'a> {
    nodes: HashMap<&'a str, ExitPaths<'a>>,
    instructions: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct ExitPaths<'a> {
    left: &'a str,
    right: &'a str,
}

fn read_network(input: &str) -> TwoPathNetwork {
    let mut nodes: HashMap<&str, ExitPaths> = HashMap::new();
    let mut instructions = "";
    for line in input.lines() {
        // if the line is empty, skip it
        // if the line contains R or L, it's the instructions line
        // if the line contains =, it's a node
        match line {
            "" => continue,
            line if line.contains("=") => {
                let mut split = line.split("=");
                let node = split.next().unwrap().trim();
                let paths = split.next().unwrap().trim();
                let mut paths = paths.split(",");
                let left = paths.next().unwrap().trim();
                let right = paths.next().unwrap().trim();
                // slice from 1 to remove the leading '('
                let left = &left[1..];
                // slice from 0 to almost end remove the trailing ')'
                let right = &right[..right.len() - 1];
                nodes.insert(node, ExitPaths { left, right });
            }
            line if line.contains("R") || line.contains("L") => instructions = line,
            _ => panic!("Invalid input line: {}", line),
        }
    }
    TwoPathNetwork {
        nodes,
        instructions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_network() {
        let input = "RL
AAA = (BBB, CCC)
BBB = (DDD, EEE)"
            .to_string();
        let network = read_network(&input);
        assert_eq!(network.instructions, "RL");
        assert_eq!(network.nodes.len(), 2);
        assert_eq!(
            network.nodes.get("AAA"),
            Some(&ExitPaths {
                left: "BBB",
                right: "CCC"
            })
        );
        assert_eq!(
            network.nodes.get("BBB"),
            Some(&ExitPaths {
                left: "DDD",
                right: "EEE"
            })
        );
    }
}
