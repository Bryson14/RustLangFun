use crate::utils::read_data;
use std::collections::{HashMap, HashSet};

const FILE: &str = "day23.txt";
const DAY: &str = "{{ DAY 23 }}";

/// --- Day 23: Unstable Diffusion ---
/// Using a HashSet to remember the position of the elves
pub fn part1() {
    let data = read_data(FILE);
    let mut elves: HashSet<Pos> = read_elves_pos(data);
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
pub fn part2() {
    let data = read_data(FILE);
}

fn plan_elf_moves(elves: &HashSet<Pos>) -> HashMap<Pos, Pos> {
    todo!()
}

fn move_elves(elves: &mut HashSet<Pos>, plan: HashMap<Pos, Pos>) {}

fn no_neighbors(elves: &HashSet<Pos>) -> bool {
    elves
        .iter()
        .map(|e| {
            elves
                .iter()
                .filter(|&o| o != e)
                .filter(|o| e.are_neighbors(o))
                .count()
        })
        .sum::<usize>()
        == 0
}

fn read_elves_pos(data: String) -> HashSet<Pos> {
    let mut elves: HashSet<Pos> =
        HashSet::with_capacity(data.chars().filter(|&c| c == '#').count());
    data.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, v)| {
            if v == '#' {
                elves.insert(Pos {
                    x: col as i64,
                    y: row as i64,
                });
            }
        });
    });
    elves
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn are_neighbors(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_elves() {
        let data = ".....\n..##.\n..#..\n.....\n..##.\n.....";
        let elves = HashSet::from_iter(
            vec![
                Pos { x: 2, y: 4 },
                Pos { x: 2, y: 1 },
                Pos { x: 2, y: 2 },
                Pos { x: 3, y: 1 },
                Pos { x: 3, y: 4 },
            ]
            .into_iter(),
        );
        assert_eq!(elves, read_elves_pos(data.into()));
    }

    #[test]
    fn test_no_neighbors() {
        let data = ".....\n..##.\n..#..\n.....\n..##.\n.....";
        println!("{:?}", read_elves_pos(data.into()));
        assert!(false);

        "..#..\n....#\n#....\n....#\n.....\n..#..\n"
    }
}
