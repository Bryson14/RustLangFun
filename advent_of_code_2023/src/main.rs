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
        1 => day01::run,
        2 => day02::run,
        3 => day03::run,
        4 => day04::run,
        5 => day05::run,
        6 => day06::run,
        7 => day07::run,
        8 => day08::run,
        9 => day09::run,
        10 => day10::run,
        11 => day11::run,
        12 => day12::run,
        13 => day13::run,
        14 => day14::run,
        15 => day15::run,
        16 => day16::run,
        17 => day17::run,
        18 => day18::run,
        19 => day19::run,
        20 => day20::run,
        21 => day21::run,
        22 => day22::run,
        23 => day23::run,
        24 => day24::run,
        25 => day25::run,
        _ => {
            eprintln!("Error: Invalid day number");
            std::process::exit(1);
        }
    };
    
    // Call the day_func with the optional part parameter
    day_func(part);
}
