#![allow(unused)]
use std::collections::HashSet;
use std::fmt;

use crate::utils::read_data;

const FILE: &str = "day9.txt";
const DAY: &str = "{{ DAY 9 }}";

/// --- Day 9: Rope Bridge ---
/// After simulating the rope, you can count up all of the positions
/// the tail visited at least once. In this diagram, s again marks the
/// starting position (which the tail also visited) and # marks other positions the tail visited:
///
/// ..##..
/// ...##.
/// .####.
/// ....#.
/// s###..
/// So, there are 13 positions the tail visited at least once.
///
/// Simulate your complete hypothetical series of motions. How many positions does the tail of the rope visit at least once?
pub fn part1() {
    let data = read_data(FILE);
    let moves = read_moves(data);
    let map_limits = get_map_limits(&moves);
    let map = move_rope(moves, map_limits, 2);
    let ans = count_tail_visits(map);
    println!("{DAY}-1 the number of cells visited by the tail are {ans}");
}

///
pub fn part2() {
    let data = read_data(FILE);
}

fn count_tail_visits(map: Vec<Vec<MapCell>>) -> usize {
    map.iter()
        .flatten()
        .filter(|c| c.knot_visits[1] > 0)
        .count()
}

fn move_rope(moves: Vec<HeadMove>, map_limits: MapLimits, num_knots: usize) -> Vec<Vec<MapCell>> {
    let mut knot_locs: Vec<(usize, usize)> = vec![
        (
            map_limits.min_x.abs() as usize,
            map_limits.max_y.abs() as usize,
        );
        num_knots
    ];
    let mut prev_loc = knot_locs.clone();
    let map_width = map_limits.max_x + 1 - map_limits.min_x;
    let map_height = map_limits.max_y + 1 - map_limits.min_y;

    let mut map: Vec<Vec<MapCell>> =
        vec![vec![MapCell::new(num_knots); map_width as usize]; map_height as usize];
    map[knot_locs[0].1][knot_locs[0].0].add_knot(0);
    map[knot_locs[1].1][knot_locs[1].0].add_knot(1);

    for m in moves.iter() {
        let steps = match m {
            HeadMove::Up(step) => *step,
            HeadMove::Down(step) => *step,
            HeadMove::Left(step) => *step,
            HeadMove::Right(step) => *step,
        };
        for _ in 0..steps {
            prev_loc[0] = (knot_locs[0].0, knot_locs[0].1);
            map[knot_locs[0].1][knot_locs[0].0].remove_knot(0);
            match m {
                HeadMove::Up(step) => knot_locs[0].1 -= 1, // down because 0,0 is at top left of map
                HeadMove::Down(step) => knot_locs[0].1 += 1,
                HeadMove::Left(step) => knot_locs[0].0 -= 1,
                HeadMove::Right(step) => knot_locs[0].0 += 1,
            };

            map.get_mut(knot_locs[0].1)
                .expect(&format!("Y index {} of h: {}", knot_locs[0].1, map_height))
                .get_mut(knot_locs[0].0)
                .expect(&format!("X index {} of w: {}", knot_locs[0].0, map_width))
                .add_knot(1);

            let diff_x = (knot_locs[0].0 as i32) - (knot_locs[1].0 as i32);
            let diff_y = (knot_locs[0].1 as i32) - (knot_locs[1].1 as i32);

            if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
            }
            // head is within '1' block of tail and doens't pull it
            else {
                map[knot_locs[1].1][knot_locs[1].0].remove_knot(1);
                map[prev_loc[0].1][prev_loc[0].0].add_knot(1);
                (knot_locs[1].0, knot_locs[1].1) = prev_loc[0];
            }

            for row in map.iter() {
                for cell in row.iter() {
                    print!("{}", cell);
                }
                println!("");
            }
            println!("");
        }
    }
    map
}

#[derive(Clone, Debug)]
struct MapCell {
    contains_knot: Vec<bool>,
    knot_visits: Vec<usize>,
}

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sign = '.';
        if self.knot_visits[0] > 0 && self.knot_visits[1] > 0 {
            sign = '#'
        } else if self.knot_visits[0] > 0 {
            sign = 'O'
        }
        write!(f, "{} ", sign)
    }
}

impl MapCell {
    fn new(knots: usize) -> Self {
        MapCell {
            contains_knot: vec![false; knots],
            knot_visits: vec![0; knots],
        }
    }

    fn add_knot(&mut self, knot: usize) {
        self.contains_knot[knot] = true;
        self.knot_visits[knot] += 1;
    }

    fn remove_knot(&mut self, knot: usize) {
        self.contains_knot[knot] = false;
    }
}

#[derive(Debug)]
struct MapLimits {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

/// returns the max width and height needed for all the movements of the head
fn get_map_limits(moves: &Vec<HeadMove>) -> MapLimits {
    let mut min_x: i32 = 0;
    let mut max_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_y: i32 = 0;
    let mut curr: (i32, i32) = (0, 0);

    // assuming the head starts at 0,0
    for m in moves {
        match m {
            HeadMove::Up(step) => curr.1 += *step as i32,
            HeadMove::Down(step) => curr.1 -= *step as i32,
            HeadMove::Left(step) => curr.0 -= *step as i32,
            HeadMove::Right(step) => curr.0 += *step as i32,
        }

        if curr.0 > max_x {
            max_x = curr.0;
        }
        if curr.0 < min_x {
            min_x = curr.0;
        }
        if curr.1 > max_y {
            max_y = curr.1;
        }
        if curr.1 < min_y {
            min_y = curr.1;
        }
    }

    MapLimits {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}

#[derive(Debug)]
enum HeadMove {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

fn read_moves(data: String) -> Vec<HeadMove> {
    data.lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut s = line.split(" ");
            let move_char = s.next();
            let steps = s.next();
            let step = steps
                .expect(&format!("Error getting next char in {line}"))
                .parse::<usize>()
                .expect(&format!("Error parsing {:?} from '{}'", steps, line));
            match move_char {
                Some("U") => Some(HeadMove::Up(step)),
                Some("D") => Some(HeadMove::Down(step)),
                Some("L") => Some(HeadMove::Left(step)),
                Some("R") => Some(HeadMove::Right(step)),
                None => None,
                _ => unreachable!(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_rope() {
        let data = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let moves = read_moves(data.into());
        let map_limits = get_map_limits(&moves);
        let map = move_rope(moves, map_limits, 2);
        let ans = count_tail_visits(map);
        assert_eq!(ans, 13);
    }
}
