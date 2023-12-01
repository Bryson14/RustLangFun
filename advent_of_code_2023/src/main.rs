use std::env;
mod utils;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the program name) is provided
    if args.len() == 1 {
        // No arguments provided, run all functions with part 0
        for day_number in 1..=25 {
            run_day(day_number, Some(0));
            run_day(day_number, Some(1));
        }
        std::process::exit(0);
    }

    // Check if the correct number of arguments is provided
    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: {} <day> [part] [test]", args[0]);
        std::process::exit(1);
    }

    // Parse the "day" argument
    let day: u32 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: 'day' must be a valid number");
            std::process::exit(1);
        }
    };

    // Parse the optional "part" argument
    let part: Option<u32> = args.get(2).map(|s| s.parse().unwrap_or_else(|_| {
        eprintln!("Error: 'part' must be a valid number");
        std::process::exit(1);
    }));

    // Parse the optional "test" argument
    // check if the argument is "test", else default to false
    let test_mode: bool = args.get(3).map_or(false, |arg| arg == "test");
    if (test_mode ) {
        println!("Running in test mode");
    }

    // Call the run_day function with the provided day and part
    run_day(day, part, test_mode);
}

fn run_day(day: u32, part: Option<u32>, test_mode: bool) {
    // Your program logic here, using 'day' and 'part' as needed
    println!("Day: {}", day);
    if let Some(part) = part {
        println!("Part: {}", part);
    } else {
        println!("Part: Not specified");

    }

    // Call the day's run function
    match day {
        1 => day01::run(part, test_mode),
        // 2 => day02::run(part, test_mode),
        // 3 => day03::run(part, test_mode),
        // 4 => day04::run(part, test_mode),
        // 5 => day05::run(part, test_mode),
        // 6 => day06::run(part, test_mode),
        // 7 => day07::run(part, test_mode),
        // 8 => day08::run(part, test_mode),
        // 9 => day09::run(part, test_mode),
        // 10 => day10::run(part, test_mode),
        // 11 => day11::run(part, test_mode),
        // 12 => day12::run(part, test_mode),
        // 13 => day13::run(part, test_mode),
        // 14 => day14::run(part, test_mode),
        // 15 => day15::run(part, test_mode),
        // 16 => day16::run(part, test_mode),
        // 17 => day17::run(part, test_mode),
        // 18 => day18::run(part, test_mode),
        // 19 => day19::run(part, test_mode),
        // 20 => day20::run(part, test_mode),
        // 21 => day21::run(part, test_mode),
        // 22 => day22::run(part, test_mode),
        // 23 => day23::run(part, test_mode),
        // 24 => day24::run(part, test_mode),
        // 25 => day25::run(part, test_mode),
        _ => {
            eprintln!("Error: Invalid day number");
            std::process::exit(1);
        }
    };
}