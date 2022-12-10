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
        if check_cycle(&peek_cycles, c) {
            println!(
                "add {i} line, str: {}, ins: {:?}",
                cpu.get_signal_strength(),
                ins
            );
            sum += cpu.get_signal_strength();
        }
        match ins {
            Instruction::Noop => {}
            Instruction::Addx(num) => {
                let c = cpu.increment_cycle();
                cpu.add_register(*num);
                if check_cycle(&peek_cycles, c) {
                    println!(
                        "add {i} line, str: {}, ins: {:?}",
                        cpu.get_signal_strength(),
                        ins
                    );
                    sum += cpu.get_signal_strength();
                }
            }
        };
    }

    println!("{DAY}-1 Sum = {sum} for {:?} cycles", peek_cycles);
}

pub fn part2() {
    let data = read_data(FILE);
}

fn check_cycle(peek_cycles: &Vec<usize>, curr_cycle: usize) -> bool {
    peek_cycles.contains(&curr_cycle)
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
                let num = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
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
