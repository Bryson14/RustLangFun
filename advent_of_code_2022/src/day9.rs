#![allow(unused)]
use std::collections::HashSet;
use std::fmt;

use crate::utils::read_data;

const FILE: &str = "day9.txt";
const DAY: &str = "{{ DAY 9 }}";

/// --- Day 9: Rope Bridge ---
pub fn part1() {
    let data = read_data(FILE);
    let moves = read_moves(data);
    let map_limits = get_map_limits(&moves);
    let map = move_rope(moves, map_limits);
    let ans = count_tail_visits(map);
    println!("{DAY}-1 the number of cells visited by the tail are {ans}");
}

pub fn part2() {
    let data = read_data(FILE);
}

fn count_tail_visits(map: Vec<Vec<MapCell>>) -> usize {
    map.iter().flatten().filter(|c| c.tail_visits > 0).count()
}

fn move_rope(moves: Vec<HeadMove>, map_limits: MapLimits) -> Vec<Vec<MapCell>> {
    let mut head_loc: (usize, usize) = (
        map_limits.min_x.abs() as usize,
        map_limits.max_y.abs() as usize,
    );
    let mut tail_loc = head_loc.clone();
    let map_width = map_limits.max_x + 1 - map_limits.min_x;
    let map_height = map_limits.max_y + 1 - map_limits.min_y;

    let mut map: Vec<Vec<MapCell>> =
        vec![vec![MapCell::new(); map_width as usize]; map_height as usize];
    map[head_loc.1][head_loc.0].add_head();
    map[tail_loc.1][tail_loc.0].add_tail();

    for m in moves.iter() {
        let steps = match m {
            HeadMove::Up(step) => *step,
            HeadMove::Down(step) => *step,
            HeadMove::Left(step) => *step,
            HeadMove::Right(step) => *step,
        };
        for _ in 0..steps {
            let previous_head = (head_loc.0, head_loc.1);
            map[head_loc.1][head_loc.0].remove_head();
            match m {
                HeadMove::Up(step) => head_loc.1 -= 1, // down because 0,0 is at top left of map
                HeadMove::Down(step) => head_loc.1 += 1,
                HeadMove::Left(step) => head_loc.0 -= 1,
                HeadMove::Right(step) => head_loc.0 += 1,
            };

            map.get_mut(head_loc.1)
                .expect(&format!("Y index {} of h: {}", head_loc.1, map_height))
                .get_mut(head_loc.0)
                .expect(&format!("X index {} of w: {}", head_loc.0, map_width))
                .add_head();

            let diff_x = (head_loc.0 as i32) - (tail_loc.0 as i32);
            let diff_y = (head_loc.1 as i32) - (tail_loc.1 as i32);

            if diff_x.abs() <= 1 && diff_y.abs() <= 1 {
            }
            // head is within '1' block of tail and doens't pull it
            else {
                map[tail_loc.1][tail_loc.0].remove_tail();
                map[previous_head.1][previous_head.0].add_tail();
                (tail_loc.0, tail_loc.1) = previous_head;
            }

            // for row in map.iter() {
            //     for cell in row.iter() {
            //         print!("{}", cell);
            //     }
            //     println!("")
            // }
            // println!("")
        }
    }
    map
}

#[derive(Copy, Clone, Debug)]
struct MapCell {
    contains_head: bool,
    contains_tail: bool,
    head_visits: usize,
    tail_visits: usize,
}

impl fmt::Display for MapCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sign = '.';
        if self.head_visits > 0 && self.tail_visits > 0 {
            sign = '#'
        } else if self.head_visits > 0 {
            sign = 'O'
        }
        write!(f, "{} ", sign)
    }
}

impl MapCell {
    fn new() -> Self {
        MapCell {
            contains_head: false,
            contains_tail: false,
            head_visits: 0,
            tail_visits: 0,
        }
    }

    fn add_head(&mut self) {
        self.contains_head = true;
        self.head_visits += 1;
    }

    fn add_tail(&mut self) {
        self.contains_tail = true;
        self.tail_visits += 1;
    }

    fn remove_head(&mut self) {
        self.contains_head = false;
    }

    fn remove_tail(&mut self) {
        self.contains_tail = false;
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
        let map = move_rope(moves, map_limits);
        let ans = count_tail_visits(map);
        assert_eq!(ans, 13);
    }
}
