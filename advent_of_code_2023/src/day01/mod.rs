// src/day01/mod.rs

mod part1;
mod part2;
mod input;

pub fn run(part: Option<u32>, test_mode: bool) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data: String;
    let example_answer: String;

    if (test_mode) {
        input_data = input::get_example_input();
        example_answer = input::get_example_answer();
        
    } else {
        input_data = input::get_input();
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

    if (test_mode) {
        println!("Example Answer: {}", example_answer);
    }
}