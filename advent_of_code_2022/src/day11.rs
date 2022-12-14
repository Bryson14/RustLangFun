#![allow(unused)]
use std::num;

use regex::Regex;

use crate::utils::read_data;

const FILE: &str = "day11.txt";
const DAY: &str = "{{ DAY 11 }}";
const PRINT: bool = true;

/// --- Day 11: Monkey in the Middle ---
pub fn part1() {
    let data = read_data(FILE);
    let mut monkeys = read_monkeys(data);
    for round in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].has_items() {
                let (to_monkey, item) = monkeys[i].inspect_item();
                monkeys[to_monkey].take_item(item);
            }
        }
    }
    for monkey in monkeys.iter() {
        monkey.report()
    }
    let mut all_items: Vec<usize> = monkeys.iter().map(|m| m.items_inspected).collect();
    all_items.sort();
    let ans: usize = all_items.iter().rev().take(2).product();

    println!("{DAY} monkey shenanigans level is {}", ans);
}

pub fn part2() {
    let data = read_data(FILE);
}

struct Monkey {
    id: usize,
    op: Box<dyn Fn(usize) -> usize>,
    items: Vec<usize>,
    test: WorryTest,
    items_inspected: usize,
}

impl Monkey {
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn inspect_item(&mut self) -> (usize, usize) {
        let num_items = self.items.len();
        self.items_inspected += num_items;
        if PRINT {
            println!("Monkey {}", self.id);
        }

        let item = self.items.pop().unwrap();
        if PRINT {
            println!("Monkey inspects an item with a worry level of {}.", item);
        }
        let new_item = (self.op)(item);
        let bored_item = new_item / 3;
        if PRINT {
            println!("Worry level is multiplied by {} to {}.", item, new_item);
            println!(
                "Monkey gets bored with item. Worry level is divided by 3 to {}.",
                bored_item
            );
        }
        let mut to_monkey = 0;
        if bored_item % self.test.divisor == 0 {
            to_monkey = self.test.if_true;
        } else {
            to_monkey = self.test.if_false;
        }
        if PRINT {
            println!(
                "Item with worry level {} is thrown to monkey {}.",
                bored_item, to_monkey
            );
        }
        (to_monkey, bored_item)
    }

    fn take_item(&mut self, item: usize) {
        self.items.push(item);
    }

    fn report(&self) {
        println!(
            "Monkey {} inspected items {} times.",
            self.id, self.items_inspected
        );
    }
}

struct WorryTest {
    if_true: usize,
    if_false: usize,
    divisor: usize,
}

fn read_monkeys(data: String) -> Vec<Monkey> {
    let monkey_re = Regex::new(r"Monkey (\d):").unwrap();
    let items_re = Regex::new(r"Starting items:(.+)$").unwrap();
    let op_re = Regex::new(r"Operation: new = old ([\*\+]) (\d{1,2})").unwrap();
    let test_re = Regex::new(r"Test: divisible by (\d{1,2})$").unwrap();
    let true_re = Regex::new(r"If true: throw to monkey (\d)").unwrap();
    let false_re = Regex::new(r"If false: throw to monkey (\d)").unwrap();

    let mut monkeys = Vec::new();
    let mut curr_idx = 0;
    let mut id = 0;
    let mut items: Vec<usize> = Vec::new();
    let mut op: Box<dyn Fn(usize) -> usize> = Box::new(|x: usize| x + 0);
    let mut divisor = 5;
    let mut if_true = 1;
    let mut if_false = 2;

    for line in data.lines() {
        let line = line.trim();
        if monkey_re.is_match(line) {
            let caps = monkey_re.captures(line).unwrap();
            id = caps
                .get(1)
                .expect("no qty number found")
                .as_str()
                .parse::<usize>()
                .expect("Cannot parse id");
        } else if items_re.is_match(line) {
            let caps = items_re.captures(line).unwrap();
            let item_str = caps.get(1).expect("no qty number found").as_str();
            let items = item_str
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        } else if op_re.is_match(line) {
            let caps = op_re.captures(line).unwrap();
            let operator = caps.get(1).expect("no sign found number found").as_str();
            let num = caps
                .get(2)
                .expect("no op divisor number found")
                .as_str()
                .parse::<usize>()
                .expect("Cannot parse op");

            if operator == "*" {
                op = Box::new(|x: usize| x * num);
            } else if operator == "+" {
                op = Box::new(|x: usize| x + num);
            } else {
                unimplemented!()
            }
        } else if test_re.is_match(line) {
            let caps = test_re.captures(line).unwrap();
            divisor = caps
                .get(1)
                .expect("no test number found")
                .as_str()
                .parse::<usize>()
                .expect("Cannot parse test num");
        } else if true_re.is_match(line) {
            let caps = true_re.captures(line).unwrap();
            if_true = caps
                .get(1)
                .expect("no if true number found")
                .as_str()
                .parse::<usize>()
                .expect("Cannot parse if true monkey id");
        } else if false_re.is_match(line) {
            let caps = false_re.captures(line).unwrap();
            if_false = caps
                .get(1)
                .expect("no if true number found")
                .as_str()
                .parse::<usize>()
                .expect("Cannot parse if false monkey id");
        } else {
            // black line between monkeys
            monkeys.push(Monkey {
                id: id.clone(),
                op: op.clone(),
                items: items.clone(),
                test: WorryTest {
                    if_true: if_true.clone(),
                    if_false: if_false.clone(),
                    divisor: divisor.clone(),
                },
                items_inspected: 0,
            })
        }
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3
      
      Monkey 1:
        Starting items: 54, 65, 75, 74
        Operation: new = old + 6
        Test: divisible by 19
          If true: throw to monkey 2
          If false: throw to monkey 0
      
      Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3
      
      Monkey 3:
        Starting items: 74
        Operation: new = old + 3
        Test: divisible by 17
          If true: throw to monkey 0
          If false: throw to monkey 1"
            .to_string();

        let monkeys = read_monkeys(input);
    }
}
