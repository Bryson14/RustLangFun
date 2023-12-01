// src/day01/mod.rs

mod part1;
mod part2;
mod input;

pub fn run(part: Option<&str>) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data = input::read_input();

    match part {
        Some("part1") => {
            println!("Running Part 1");
            part1::solve(&input_data);
        }
        Some("part2") => {
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
}