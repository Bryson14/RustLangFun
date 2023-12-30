// src/day01/mod.rs

use crate::utils;
mod part1;
mod part2;

pub fn run(part: Option<u32>, test_mode: bool) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data: String;
    let mut example_answer: String = String::from("None");
    const DAY: &str = "09";

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

fn read_history_line(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

fn read_all_history(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(read_history_line).collect()
}

// take the history and find the difference between each number,
// add that to a new list
// repeat until the list is all zeros
fn create_diff_map(history: &[isize]) -> Vec<Vec<isize>> {
    let mut diff_map = Vec::new();
    diff_map.push(history.to_vec());
    let mut curr_list = 0;
    loop {
        let mut new_diffs = Vec::new();
        for i in 0..diff_map[curr_list].len() - 1 {
            // find the diffs and add them to the new list
            let diff = diff_map[curr_list][i + 1] - diff_map[curr_list][i];
            new_diffs.push(diff);
        }

        // add the new list to the diff map
        diff_map.push(new_diffs);
        curr_list += 1;

        // if the new list is all zeros, we are done
        if diff_map[curr_list].iter().all(|&x| x == 0) {
            break;
        }
    }
    diff_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_history_line() {
        let input = "35 -5 16";
        let expected = vec![35, -5, 16];
        assert_eq!(read_history_line(input), expected);
    }

    #[test]
    fn test_read_history_line2() {
        let input = "8 10 9 -2 -17 8 190 771 2208 5345 11753 24409 49063 97014 190779 375658 743112 1477240 2945141 5868120 11637263";
        let expected = vec![
            8, 10, 9, -2, -17, 8, 190, 771, 2208, 5345, 11753, 24409, 49063, 97014, 190779, 375658,
            743112, 1477240, 2945141, 5868120, 11637263,
        ];
        assert_eq!(read_history_line(input), expected);
    }

    #[test]
    fn test_create_diff_map() {
        let input = vec![0, 3, 6, 9, 12, 15];
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(create_diff_map(&input), expected);
    }

    #[test]
    fn test_create_diff_map2() {
        let input = vec![1, 3, 6, 10, 15, 21];
        let expected = vec![
            vec![1, 3, 6, 10, 15, 21],
            vec![2, 3, 4, 5, 6],
            vec![1, 1, 1, 1],
            vec![0, 0, 0],
        ];
        assert_eq!(create_diff_map(&input), expected);
    }
}
