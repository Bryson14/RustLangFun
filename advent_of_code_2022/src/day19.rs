#![allow(unused)]
use regex::Regex;
use std::fmt;

use crate::utils::read_data;

const FILE: &str = "day19.txt";
const DAY: &str = "{{ DAY 19 }}";

/// --- Day 19: Not Enough Minerals ---
/// However, by using blueprint 2 in the example above, you could do even better:
/// the largest number of geodes you could open in 24 minutes is 12.
///
/// Determine the quality level of each blueprint by multiplying that blueprint's
/// ID number with the largest number of geodes that can be opened in 24 minutes using
/// that blueprint. In this example, the first blueprint has ID 1 and can open 9 geodes,
/// so its quality level is 9. The second blueprint has ID 2 and can open 12 geodes,
/// so its quality level is 24. Finally, if you add up the quality levels of all of
/// the blueprints in the list, you get 33.
///
/// Determine the quality level of each blueprint using the largest number of geodes
/// it could produce in 24 minutes. What do you get if you add up the quality level of
/// all of the blueprints in your list?
pub fn part1() {
    let data = read_data(FILE);
    let blueprints = parse_blueprint(data);
    let ans = blueprints
        .iter()
        .map(|bp| calculate_geode(bp) * bp.id)
        .sum::<usize>();
    println!("{DAY}-1 The sum of the quality points of each blueprint is {ans}");
}

pub fn part2() {
    let data = read_data(FILE);
}

fn parse_blueprint(data: String) -> Vec<Blueprint> {
    let mut blueprints = Vec::with_capacity(data.lines().count());
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
    for line in data.lines() {
        let cap = re.captures(line).unwrap();
        let id = get_usize_capture(&cap, 1);
        let ore_robot = get_usize_capture(&cap, 2);
        let clay_robot = get_usize_capture(&cap, 3);
        let obsedian_robot_ore = get_usize_capture(&cap, 4);
        let obsedian_robot_clay = get_usize_capture(&cap, 5);
        let geode_robot_ore = get_usize_capture(&cap, 6);
        let geode_robot_obsedian = get_usize_capture(&cap, 7);

        blueprints.push(Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot: (obsedian_robot_ore, obsedian_robot_clay),
            geode_robot: (geode_robot_ore, geode_robot_obsedian),
        })
    }
    blueprints
}

fn get_usize_capture(cap: &regex::Captures, idx: usize) -> usize {
    cap.get(idx)
        .expect("no qty number found")
        .as_str()
        .parse::<usize>()
        .expect("Cannot parse id")
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot: usize,               //ore
    clay_robot: usize,              // ore
    obsidian_robot: (usize, usize), // (ore, clay)
    geode_robot: (usize, usize),    // (ore, obsidian)
}

impl fmt::Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("Blueprint {}:\n", self.id));
        s.push_str(&format!("  Each ore robot costs {} ore.\n", self.ore_robot));
        s.push_str(&format!(
            "  Each clay robot costs {} ore.\n",
            self.clay_robot
        ));
        s.push_str(&format!(
            "  Each obsidian robot costs {} ore and {} clay.\n",
            self.obsidian_robot.0, self.obsidian_robot.1
        ));
        s.push_str(&format!(
            "  Each geode robot costs {} ore and {} obsidian.\n",
            self.geode_robot.0, self.geode_robot.1
        ));
        write!(f, "{}\n", s)
    }
}

fn calculate_geode(bp: &Blueprint) -> usize {
    todo!()
}

enum Material {
    Ore,
    Clay,
    Obsidian,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_blueprint() {
        let data = read_data(FILE);
        let blueprints = parse_blueprint(data);
        for bp in blueprints.iter() {
            println!("{bp}");
        }
        assert!(false)
    }
}
