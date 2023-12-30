// src/day01/mod.rs

use crate::utils;
mod input;
mod part1;
mod part2;

pub fn run(part: Option<u32>, test_mode: bool) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data: String;
    let mut example_answer: String = String::from("None");
    const DAY: &str = "07";
    println!("Not Finished");

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
