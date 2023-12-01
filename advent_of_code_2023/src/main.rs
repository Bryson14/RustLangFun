use std::env;
use advent_of_code_2023 as aoc2023;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the program name) is provided
    if args.len() < 2 || args.len() > 3{
        eprintln!("Usage: {} <day> [part]", args[0]);
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

    // Your program logic here, using 'day' and 'part' as needed
    println!("Day: {}", day);
    if let Some(part) = part {
        println!("Part: {}", part);
    } else {
        println!("Part: Not specified");
    }
 
    // start the program
    // call the day and maybe the part, or maybe just pass part as an argument to the day
    let day_func = match day {
        1 => aoc2023::day01::run,
        2 => aoc2023::day02::run,
        3 => aoc2023::day03::run,
        4 => aoc2023::day04::run,
        5 => aoc2023::day05::run,
        6 => aoc2023::day06::run,
        7 => aoc2023::day07::run,
        8 => aoc2023::day08::run,
        9 => aoc2023::day09::run,
        10 => aoc2023::day10::run,
        11 => aoc2023::day11::run,
        12 => aoc2023::day12::run,
        13 => aoc2023::day13::run,
        14 => aoc2023::day14::run,
        15 => aoc2023::day15::run,
        16 => aoc2023::day16::run,
        17 => aoc2023::day17::run,
        18 => aoc2023::day18::run,
        19 => aoc2023::day19::run,
        20 => aoc2023::day20::run,
        21 => aoc2023::day21::run,
        22 => aoc2023::day22::run,
        23 => aoc2023::day23::run,
        24 => aoc2023::day24::run,
        25 => aoc2023::day25::run,
        _ => {
            eprintln!("Error: Invalid day number");
            std::process::exit(1);
        }
    };
    
    // Call the day_func with the optional part parameter
    day_func(part);
}
