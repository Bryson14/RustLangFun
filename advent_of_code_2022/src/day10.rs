#![allow(unused)]
use crate::utils::read_data;

const FILE: &str = "day10.txt";
const DAY: &str = "{{ DAY 10 }}";

/// Day 10: Cathode Ray Tube
/// Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which starts with the value 1.
/// It supports only two instructions:
/// addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
/// noop takes one cycle to complete. It has no other effect.
pub fn part1() {
    let data = read_data(FILE);
    let instructions = read_instructions(data);
    let mut cpu = CPU::new();
    let peek_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    for (i, ins) in instructions.iter().enumerate() {
        let c = cpu.increment_cycle();
        check_add(&peek_cycles, c, &mut sum, &mut cpu);
        match ins {
            Instruction::Noop => {}
            Instruction::Addx(num) => {
                let c = cpu.increment_cycle();
                check_add(&peek_cycles, c, &mut sum, &mut cpu);
                cpu.add_register(*num);
            }
        };
        println!("reg: {}", cpu.register);
    }

    println!("{DAY}-1 Sum = {sum} for {:?} cycles", peek_cycles);
}

pub fn part2() {
    let data = read_data(FILE);
    let instructions = read_instructions(data);
    let mut cpu = CPU::new();
    let peek_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut sum = 0;
    let mut display = [[0u8; 41]; 7];
    for (i, ins) in instructions.iter().enumerate() {
        let c = cpu.increment_cycle();
        check_add(&peek_cycles, c, &mut sum, &mut cpu);
        match ins {
            Instruction::Noop => {}
            Instruction::Addx(num) => {
                let c = cpu.increment_cycle();
                check_add(&peek_cycles, c, &mut sum, &mut cpu);
                cpu.add_register(*num);
            }
        };
        println!("reg: {}", cpu.register);
    }

    let mut display_str = String::new();
    for line in display {
        display_str.push('\n');
        for pixel in line {
            display_str.push_str(if pixel == 0 { " " } else { "#" });
        }
    }

    println!("{DAY}-1 Sum = {sum} for {:?} cycles", peek_cycles);
    println!("{}", display_str);
}

fn draw_pixel_crt(display: &mut [[u8; 41]; 7], signal: i32, cycle: i32) {
    let pixel_y = ((cycle - 1) / 40);
    let pixel_x = ((cycle - 1) % 40);
    if (signal - pixel_x).abs() <= 1 {
        display[pixel_y as usize][pixel_x as usize] = 1;
    } else {
        display[pixel_y as usize][pixel_x as usize] = 0;
    }
}

fn check_cycle(peek_cycles: &Vec<usize>, curr_cycle: usize) -> bool {
    peek_cycles.contains(&curr_cycle)
}

fn check_add(peek_cycles: &Vec<usize>, curr_cycle: usize, sum: &mut i32, cpu: &mut CPU) {
    if check_cycle(&peek_cycles, curr_cycle) {
        *sum += cpu.get_signal_strength();
    }
}

struct CPU {
    register: i32,
    cycles: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register: 1,
            cycles: 0,
        }
    }

    fn get_signal_strength(&self) -> i32 {
        self.register * self.cycles as i32
    }

    fn increment_cycle(&mut self) -> usize {
        self.cycles += 1;
        self.cycles
    }

    fn add_register(&mut self, val: i32) {
        self.register += val;
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn read_instructions(data: String) -> Vec<Instruction> {
    data.lines()
        .map(|line| line.trim())
        .map(|line| {
            if line.starts_with("noop") {
                return Instruction::Noop;
            } else if line.starts_with("addx") {
                let num = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                return Instruction::Addx(num);
            } else {
                unreachable!();
            }
        })
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
