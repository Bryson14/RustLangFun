#![allow(dead_code)]
mod day1;
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
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    let completed = [1, 2, 3, 4, 5, 6];

    if !completed.contains(&1) {
        day1::part1();
        day1::part2();
    }

    if !completed.contains(&2) {
        day2::part1();
        day2::part2();
    }

    if !completed.contains(&3) {
        day3::part1();
        day3::part2();
    }

    if !completed.contains(&4) {
        day4::part1();
        day4::part2();
    }

    if !completed.contains(&5) {
        day5::part1();
        day5::part2();
    }

    if !completed.contains(&6) {
        day6::part1();
        day6::part2();
    }

    if !completed.contains(&7) {
        day7::part1();
        day7::part2();
    }

    // day8::part1();
    // day8::part2();
}
