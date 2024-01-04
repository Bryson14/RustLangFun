// src/day01/mod.rs

use std::{collections::HashMap, io::Empty};

use crate::utils;
mod part1;
mod part2;

pub fn run(part: Option<u32>, test_mode: bool) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data: String;
    let mut example_answer: String = String::from("None");
    const DAY: &str = "10";

    if test_mode {
        let part_str = match part {
            Some(1) => "1",
            Some(2) => "2",
            _ => "1",
        };
        let (example_input, ans) = utils::get_example(DAY, part_str);
        input_data = example_input;
        example_answer = ans;
    } else {
        input_data = utils::get_input(DAY);
    }

    match part {
        Some(1) => {
            println!("Running Part 1");
            part1::solve(&input_data);
        }
        Some(2) => {
            println!("Running Part 2");
            part2::solve(&input_data);
        }
        _ => {
            // Default behavior: run both parts
            println!("Running Part 1");
            part1::solve(&input_data);

            println!("Running Part 2");
            part2::solve(&input_data);
        }
    }

    if test_mode {
        println!("Example Answer: {}", example_answer);
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    ElbowNorthEast,
    ElbowNorthWest,
    ElbowSouthEast,
    ElbowSouthWest,
    Empty,
    Start,
}

impl Pipe {
    // get the relative positions of neighbors for a given pipe
    // (0,0) is top left of the map so
    // south is y + 1, north is y - 1, east is x + 1, west is x - 1
    fn get_possible_neighbors(&self) -> Vec<RelativePosition> {
        match self {
            Pipe::Vertical => vec![
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: 0, y: 1 },
            ],
            Pipe::Horizontal => vec![
                RelativePosition { x: -1, y: 0 },
                RelativePosition { x: 1, y: 0 },
            ],
            Pipe::ElbowNorthEast => vec![
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: 1, y: 0 },
            ],
            Pipe::ElbowNorthWest => vec![
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: -1, y: 0 },
            ],
            Pipe::ElbowSouthEast => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: 1, y: 0 },
            ],
            Pipe::ElbowSouthWest => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: -1, y: 0 },
            ],
            Pipe::Empty => vec![],
            Pipe::Start => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: 1, y: 0 },
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: -1, y: 0 },
            ],
        }
    }

    /// Gets the possible neighbor positions for a given position on the map
    fn get_possible_neighbor_positions(&self, pos: &Position) -> Vec<Position> {
        let neighbors = self.get_possible_neighbors();
        let mut neighbor_positions = Vec::with_capacity(4);
        for neighbor in neighbors.iter() {
            let x = pos.x as isize + neighbor.x;
            let y = pos.y as isize + neighbor.y;

            if x < 0 || y < 0 {
                continue;
            }

            neighbor_positions.push(Position {
                x: x as usize,
                y: y as usize,
            });
        }
        neighbor_positions
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct RelativePosition {
    x: isize,
    y: isize,
}

pub fn read_map(input: &str) -> HashMap<Position, Pipe> {
    let mut map = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.trim().chars() {
            let pipe = match c {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::ElbowNorthEast,
                'J' => Pipe::ElbowNorthWest,
                'F' => Pipe::ElbowSouthEast,
                '7' => Pipe::ElbowSouthWest,
                '.' => Pipe::Empty,
                'S' => Pipe::Start,
                _ => panic!("Unknown pipe type: {}", c),
            };
            map.insert(Position { x, y }, pipe);
            x += 1;
        }
        y += 1;
    }
    map
}

/// Finds the two neighbors that connect to the current position
/// Else returns None if this current position is not in the main loop of pipe
/// Convention is (x_diff, y_diff)
pub fn find_connecting_pipes(
    map: &HashMap<Position, Pipe>,
    pos: &Position,
) -> Option<Vec<Position>> {
    let pipe = map.get(pos).unwrap();
    let neighbors_pos = pipe.get_possible_neighbor_positions(pos);
    let mut neighbors = Vec::new();

    for neighbor in neighbors_pos.iter() {
        if let Some(neighbor_pipe) = map.get(neighbor) {
            let neighbor_neighbors = neighbor_pipe.get_possible_neighbor_positions(neighbor);
            if neighbor_neighbors.contains(pos) {
                // both the current node and their neighbor have each other as neighbors
                neighbors.push(*neighbor);
            }
        }
    }

    if neighbors.len() == 2 {
        return Some(neighbors);
    }

    None
}

pub fn find_start_pos(map: &HashMap<Position, Pipe>) -> Option<Position> {
    for (pos, pipe) in map.iter() {
        if *pipe == Pipe::Start {
            return Some(*pos);
        }
    }
    None
}

// prints the map into a grid
pub fn format_map(map: &HashMap<Position, Pipe>) -> String {
    let max_width = map.keys().map(|pos| pos.x).max().unwrap();
    let max_height = map.keys().map(|pos| pos.y).max().unwrap();
    let mut map_string = String::new();

    for y in 0..max_height + 1 {
        for x in 0..max_width + 1 {
            let pipe = map.get(&Position { x, y });
            match pipe {
                Some(Pipe::Vertical) => map_string.push_str("|"),
                Some(Pipe::Horizontal) => map_string.push_str("-"),
                Some(Pipe::ElbowNorthEast) => map_string.push_str("L"),
                Some(Pipe::ElbowNorthWest) => map_string.push_str("J"),
                Some(Pipe::ElbowSouthEast) => map_string.push_str("F"),
                Some(Pipe::ElbowSouthWest) => map_string.push_str("7"),
                Some(Pipe::Empty) => map_string.push_str("."),
                Some(Pipe::Start) => map_string.push_str("S"),
                None => map_string.push_str(" "),
            }
        }
        map_string.push_str("\n");
    }

    map_string
}

pub fn format_dis_map(map: &HashMap<Position, usize>) -> String {
    let max_width = map.keys().map(|pos| pos.x).max().unwrap();
    let max_height = map.keys().map(|pos| pos.y).max().unwrap();
    let max_dis = map.values().max().unwrap();
    let mut map_string = String::new();

    // pad everything to be the length of the max_dis
    let max_dis_len = max_dis.to_string().len() + 1;

    for y in 0..max_height + 1 {
        for x in 0..max_width + 1 {
            let pipe = map.get(&Position { x, y });
            match pipe {
                // pad with max_dis_len
                Some(dis) => map_string.push_str(&format!("{:width$}", dis, width = max_dis_len)),
                None => map_string.push_str(&format!("{:width$}", '.', width = max_dis_len)),
            }
        }
        map_string.push_str("\n");
    }
    map_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_map() {
        let input = ".....
        .F-7.
        .|.|.
        .L-J.
        .....";

        let map = read_map(input);
        assert_eq!(map[&Position { x: 0, y: 0 }], Pipe::Empty);
        assert_eq!(map[&Position { x: 1, y: 1 }], Pipe::ElbowSouthEast);
        assert_eq!(map[&Position { x: 2, y: 1 }], Pipe::Horizontal);
        assert_eq!(map[&Position { x: 3, y: 1 }], Pipe::ElbowSouthWest);
        assert_eq!(map[&Position { x: 1, y: 2 }], Pipe::Vertical);
        assert_eq!(map[&Position { x: 3, y: 2 }], Pipe::Vertical);
        assert_eq!(map[&Position { x: 1, y: 3 }], Pipe::ElbowNorthEast);
        assert_eq!(map[&Position { x: 2, y: 3 }], Pipe::Horizontal);
        assert_eq!(map[&Position { x: 3, y: 3 }], Pipe::ElbowNorthWest);
        assert_eq!(map[&Position { x: 4, y: 4 }], Pipe::Empty);
    }

    #[test]
    fn test_find_connecting_pipes() {
        let input = ".....
        .F-7.
        .|.|.
        .L-J.
        .....";

        let map = read_map(input);
        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 2, y: 1 }));
        assert!(neighbors.contains(&Position { x: 1, y: 2 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 2, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 1 }));
        assert!(neighbors.contains(&Position { x: 3, y: 1 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 3, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 2, y: 1 }));
        assert!(neighbors.contains(&Position { x: 3, y: 2 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 2 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 1 }));
        assert!(neighbors.contains(&Position { x: 1, y: 3 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 3, y: 2 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 3, y: 1 }));
        assert!(neighbors.contains(&Position { x: 3, y: 3 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 3 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 2 }));
        assert!(neighbors.contains(&Position { x: 2, y: 3 }));

        // test the top left corner with no neightbors
        let neighbors = find_connecting_pipes(&map, &Position { x: 0, y: 0 });
        assert!(neighbors.is_none());
    }

    #[test]
    fn test_find_connecting_pipes_2() {
        let input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

        let map = read_map(input);
        let neighbors = find_connecting_pipes(&map, &Position { x: 0, y: 2 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 0, y: 3 }));
        assert!(neighbors.contains(&Position { x: 1, y: 2 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 2 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 0, y: 2 }));
        assert!(neighbors.contains(&Position { x: 1, y: 1 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 2 }));
        assert!(neighbors.contains(&Position { x: 2, y: 1 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 2, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 1 }));
        assert!(neighbors.contains(&Position { x: 2, y: 0 }));
    }

    #[test]
    fn test_find_connecting_pipes_3() {
        // testing that it can deal with pipes that don't connect to anything
        let input = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";

        let map = read_map(input);
        let neighbors = find_connecting_pipes(&map, &Position { x: 0, y: 2 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 0, y: 3 }));
        assert!(neighbors.contains(&Position { x: 1, y: 2 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 2 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 0, y: 2 }));
        assert!(neighbors.contains(&Position { x: 1, y: 1 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 1, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 2 }));
        assert!(neighbors.contains(&Position { x: 2, y: 1 }));

        let neighbors = find_connecting_pipes(&map, &Position { x: 2, y: 1 }).unwrap();
        assert_eq!(neighbors.len(), 2);
        assert!(neighbors.contains(&Position { x: 1, y: 1 }));
        assert!(neighbors.contains(&Position { x: 2, y: 0 }));
    }

    #[test]
    fn test_find_start_pos() {
        let input = ".....
        .F-7.
        .|.|.
        .L-J.
        .....";

        let map = read_map(input);
        let start_pos = find_start_pos(&map);
        assert!(start_pos.is_none());
    }

    #[test]
    fn test_find_start_pos_with_start() {
        let input = ".....
        .F-7.
        .|.|.
        .L-J.
        .S...";

        let map = read_map(input);
        let start_pos = find_start_pos(&map).unwrap();
        assert_eq!(start_pos, Position { x: 1, y: 4 });
    }
}
