#![allow(unused)]
use crate::utils::read_data;
use regex::Regex;
use std::{collections::HashMap, hash::Hash};

const FILE: &str = "day16.txt";
const DAY: &str = "{{ DAY 16 }}";

/// --- Day 16: Proboscidea Volcanium ---
/// Its a dynamic programming problem
/// Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can release?
pub fn part1() {
    let data = read_data(FILE);
    let valves = read_valves_and_tunnels(data.into());
    let map = convert_vec_to_hashmap(&valves);
    let max_p = find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 30);

    println!("{DAY}-1 Max pressure released is {max_p}");
}

/// You're worried that even with an optimal approach, the pressure released won't be enough.
/// What if you got one of the elephants to help you?
/// With the elephant helping, after 26 minutes, the best you could do would release a total of 1707 pressure.
///
/// With you and an elephant working together for 26 minutes, what is the most pressure you could release?
pub fn part2() {
    let data = read_data(FILE);
}

fn find_max_pressure_memo_elephant(
    valves: &HashMap<usize, Valve>,
    str_int_map: &HashMap<&String, usize>,
    memo: &mut HashMap<MemoizedParam, usize>,
    param: MemoizedParam,
) -> usize {
    if memo.get(&param).is_some() {
        return *memo.get(&param).unwrap();
    }
    if param.time <= 0 {
        memo.insert(param.clone(), 0);
        return 0;
    }
    // moving to new valve or opening wont do anything
    if param.time == 1 {
        memo.insert(param.clone(), param.flow);
        return param.flow;
    }

    let mut options = Vec::new();
    // cannot open opened valve
    if !param.opened.contains(param.pos) {
        // skipping blocked valves
        if valves.get(&param.pos).unwrap().flow_rate != 0 {
            let mut new_opened = param.opened.add_visit(param.pos);
            options.push(
                find_max_pressure_memo(
                    valves,
                    str_int_map,
                    memo,
                    MemoizedParam {
                        flow: param.flow + valves.get(&param.pos).unwrap().flow_rate,
                        pos: param.pos,
                        time: param.time - 1,
                        opened: new_opened,
                    },
                ) + param.flow,
            );
        }
    }

    for v in valves.get(&param.pos).unwrap().tunnels.iter() {
        let new_pos: usize = *str_int_map.get(v).unwrap();
        options.push(
            find_max_pressure_memo(
                valves,
                str_int_map,
                memo,
                MemoizedParam {
                    flow: param.flow,
                    pos: new_pos,
                    time: param.time - 1,
                    opened: param.opened.clone(),
                },
            ) + param.flow,
        );
    }

    let max_release = *options.iter().max().unwrap();
    memo.insert(param.clone(), max_release);
    max_release
}

fn find_max_pressure_memo_helper(
    valves: &HashMap<String, Valve>,
    curr_pos: String,
    curr_flow: usize,
    opened: Vec<String>,
    time_remaining: usize,
) -> usize {
    let mut string_int_name_map: HashMap<&String, usize> = HashMap::new();
    let mut int_valves: HashMap<usize, Valve> = HashMap::new();
    valves.iter().enumerate().for_each(|(i, (k, v))| {
        string_int_name_map.insert(k, i);
        int_valves.insert(i, v.clone());
    });
    let pos_int = string_int_name_map.get(&curr_pos).unwrap();
    let param = MemoizedParam {
        flow: curr_flow,
        pos: *pos_int,
        time: time_remaining,
        opened: OpenedString { opened: "".into() },
    };
    find_max_pressure_memo(
        &int_valves,
        &string_int_name_map,
        &mut HashMap::new(),
        param,
    )
}

fn find_max_pressure_memo(
    valves: &HashMap<usize, Valve>,
    str_int_map: &HashMap<&String, usize>,
    memo: &mut HashMap<MemoizedParam, usize>,
    param: MemoizedParam,
) -> usize {
    if memo.get(&param).is_some() {
        return *memo.get(&param).unwrap();
    }
    if param.time <= 0 {
        memo.insert(param.clone(), 0);
        return 0;
    }
    // moving to new valve or opening wont do anything
    if param.time == 1 {
        memo.insert(param.clone(), param.flow);
        return param.flow;
    }

    let mut options = Vec::new();
    // cannot open opened valve
    if !param.opened.contains(param.pos) {
        // skipping blocked valves
        if valves.get(&param.pos).unwrap().flow_rate != 0 {
            let mut new_opened = param.opened.add_visit(param.pos);
            options.push(
                find_max_pressure_memo(
                    valves,
                    str_int_map,
                    memo,
                    MemoizedParam {
                        flow: param.flow + valves.get(&param.pos).unwrap().flow_rate,
                        pos: param.pos,
                        time: param.time - 1,
                        opened: new_opened,
                    },
                ) + param.flow,
            );
        }
    }

    for v in valves.get(&param.pos).unwrap().tunnels.iter() {
        let new_pos: usize = *str_int_map.get(v).unwrap();
        options.push(
            find_max_pressure_memo(
                valves,
                str_int_map,
                memo,
                MemoizedParam {
                    flow: param.flow,
                    pos: new_pos,
                    time: param.time - 1,
                    opened: param.opened.clone(),
                },
            ) + param.flow,
        );
    }

    let max_release = *options.iter().max().unwrap();
    memo.insert(param.clone(), max_release);
    max_release
}

// needs to memoized or dp
fn find_max_pressure(
    valves: &HashMap<String, Valve>,
    curr_pos: String,
    curr_flow: usize,
    opened: Vec<String>,
    time_remaining: usize,
    print_output: bool,
) -> usize {
    if time_remaining <= 0 {
        if print_output {
            println!("Game over! {curr_pos} {opened:?} flow: {curr_flow}");
        }
        return 0;
    }
    // moving to new valve or opening wont do anything
    if time_remaining == 1 {
        if print_output {
            println!("Nothing left to do but wait! {curr_pos} {opened:?} flow: {curr_flow}");
        }
        return curr_flow;
    }

    let mut options = Vec::new();
    // cannot open opened valve
    if !opened.contains(&curr_pos) {
        // skipping blocked valves
        if valves.get(&curr_pos).unwrap().flow_rate != 0 {
            let mut new_opened = opened.clone();
            new_opened.push(curr_pos.clone());
            if print_output {
                println!("opening! {curr_pos} visited: {opened:?} time: {time_remaining}");
            }
            options.push(
                find_max_pressure(
                    valves,
                    curr_pos.clone(),
                    curr_flow + valves.get(&curr_pos).unwrap().flow_rate,
                    new_opened,
                    time_remaining - 1,
                    print_output,
                ) + curr_flow,
            );
        }
    }

    for v in valves.get(&curr_pos).unwrap().tunnels.iter() {
        options.push(
            find_max_pressure(
                valves,
                v.to_string(),
                curr_flow,
                opened.clone(),
                time_remaining - 1,
                print_output,
            ) + curr_flow,
        );
    }

    let max_release = *options.iter().max().unwrap();
    if print_output {
        println!("Pressue release = {max_release} pos: {curr_pos} time: {time_remaining}");
    }

    max_release
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct MemoizedParam {
    flow: usize,
    pos: usize,
    time: usize,
    opened: OpenedString,
}

struct MemoizedTunnelSearch {
    map: HashMap<MemoizedParam, usize>,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct OpenedString {
    opened: String,
}

impl OpenedString {
    fn add_visit(&self, visit: usize) -> Self {
        let mut c = self.opened.clone();
        c.push_str(&format!(" {visit}"));
        OpenedString { opened: c }
    }

    fn contains(&self, pos: usize) -> bool {
        self.opened
            .split(" ")
            .map(|s| s.trim().parse::<usize>())
            .filter(|i| i.is_ok())
            .map(|i| i.unwrap())
            .any(|v| v == pos)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn convert_vec_to_hashmap(valves: &Vec<Valve>) -> HashMap<String, Valve> {
    valves.iter().map(|v| (v.name.clone(), v.clone())).collect()
}

fn read_valves_and_tunnels(data: String) -> Vec<Valve> {
    let re =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    data.lines()
        .map(|line| {
            let caps = re
                .captures(line)
                .expect(&format!("Error with line: {line}"));
            let name = caps.get(1).expect("first capture not found").as_str();
            let flow_rate = caps
                .get(2)
                .expect("second capture not found")
                .as_str()
                .parse::<usize>()
                .expect("Cannot parse flow rate");
            let tunnel_str = caps.get(3).expect("third capture not found").as_str();
            let tunnels = tunnel_str
                .split(",")
                .map(|s| s.trim().into())
                .collect::<Vec<String>>();

            return Valve {
                name: name.into(),
                flow_rate,
                tunnels,
            };
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tunnels() {
        let data = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II";

        let valves = read_valves_and_tunnels(data.into());
        assert_eq!(
            valves[0],
            Valve {
                name: "AA".into(),
                flow_rate: 0,
                tunnels: vec!["DD".into(), "II".into(), "BB".into()]
            }
        );
        assert_eq!(
            valves[9],
            Valve {
                name: "JJ".into(),
                flow_rate: 21,
                tunnels: vec!["II".into()]
            }
        )
    }

    #[test]
    fn test_find_best_1() {
        let data = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II";

        let valves = read_valves_and_tunnels(data.into());
        let map = convert_vec_to_hashmap(&valves);
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 0, false),
            0
        ); // out of time
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 1, false),
            0
        ); // moved at ran out
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 2, false),
            0
        ); // just opened valve but ran out
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 3, false),
            20
        ); // DD valve open for 1 min, move to CC
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 4, false),
            40
        ); // DD valve open for 2 min, move to BB
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 5, true),
            63
        ); // valve open for 3 min, jumps to EE and opens it quick
        assert_eq!(
            find_max_pressure(&map, "AA".into(), 0, Vec::new(), 6, false),
            93
        ); // valve open for 4 min +
    }

    #[test]
    fn test_find_best_2_memo() {
        let data = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II";

        let valves = read_valves_and_tunnels(data.into());
        let map = convert_vec_to_hashmap(&valves);
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 0),
            0
        ); // out of time
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 1),
            0
        ); // moved at ran out
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 2),
            0
        ); // just opened valve but ran out
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 3),
            20
        ); // DD valve open for 1 min, move to CC
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 4),
            40
        ); // DD valve open for 2 min, move to BB
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 5),
            63
        ); // valve open for 3 min, jumps to EE and opens it quick
        assert_eq!(
            find_max_pressure_memo_helper(&map, "AA".into(), 0, Vec::new(), 6),
            93
        ); // valve open for 4 min +
    }
}
