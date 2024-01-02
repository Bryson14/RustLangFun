// src/day01/mod.rs

use std::collections::{HashMap, HashSet};

use crate::utils;
mod part1;
mod part2;

pub fn run(part: Option<u32>, test_mode: bool) {
    // Your code to run for the entire day
    // Call part1 and/or part2 functions based on the 'part' parameter
    let input_data: String;
    let mut example_answer: String = String::from("None");
    const DAY: &str = "11";

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


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct SpaceImage {
    pub width: usize,
    pub height: usize,
    // the value is the index of the image in the image vector keeping tracking of the images
    pub img: HashMap<Position, usize>,
}

impl SpaceImage {

    pub fn from_string(input: &str) -> SpaceImage {
        let mut img: HashMap<Position, usize> = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        let mut img_index = 0;
        width = input.lines().next().unwrap().len();
        for (y, line) in input.lines().enumerate() {
            height += 1;
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    img.insert(Position { x, y }, img_index);
                    img_index += 1;
                }
            }
        }
        SpaceImage {
            width,
            height,
            img,
        }
    }

    /// Gets the rows in the image that are empty
    pub fn get_empty_rows(&self) -> Vec<usize> {
        // make a set of all rows from 0 to height
        let all_rows: HashSet<usize> = (0..self.height).collect();
    
        // make a set of all the rows in the image
        let occupied_rows: HashSet<usize> = self.img.keys().map(|pos| pos.y).collect();
    
        // find the rows that are not in the image by getting the difference between the two sets
        let empty_rows: Vec<usize> = all_rows.difference(&occupied_rows).cloned().collect();
    
        empty_rows
    }

    /// Gets the columns in the image that are empty
    pub fn get_empty_columns(&self) -> Vec<usize> {
        let all_columns: HashSet<usize> = (0..self.width).collect();
        let occupied_columns: HashSet<usize> = self.img.keys().map(|pos| pos.x).collect();
        let empty_columns: Vec<usize> = all_columns.difference(&occupied_columns).cloned().collect();
        empty_columns
    }   

    pub fn to_string(&self) -> String {
        let mut output = String::with_capacity(self.height * self.width);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.img.contains_key(&Position { x, y }) {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }
        output
    }
}


/// calculates the distance between two positions using manhattan distance
pub fn calculate_distance(pos1: &Position, pos2: &Position) -> usize {
    let x_dist = if pos1.x > pos2.x { pos1.x - pos2.x } else { pos2.x - pos1.x };
    let y_dist = if pos1.y > pos2.y { pos1.y - pos2.y } else { pos2.y - pos1.y };
    x_dist + y_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_distance() {
        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 3, y: 4 };
        assert_eq!(calculate_distance(&pos1, &pos2), 7);
    }

    #[test]
    fn test_calculate_distance_2() {
        let pos1 = Position { x: 1, y: 6 };
        let pos2 = Position { x: 5, y: 11 };
        assert_eq!(calculate_distance(&pos1, &pos2), 9);
    }


    #[test]
    fn test_space_image_from_string() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let space_image = SpaceImage::from_string(input);
        assert_eq!(space_image.width, 10);
        assert_eq!(space_image.height, 10);
        assert_eq!(space_image.img.len(), 9);
        assert_eq!(space_image.img.get(&Position { x: 3, y: 0 }), Some(&0));
        assert_eq!(space_image.img.get(&Position { x: 7, y: 1 }), Some(&1));
        assert_eq!(space_image.img.get(&Position { x: 0, y: 2 }), Some(&2));
        assert_eq!(space_image.img.get(&Position { x: 6, y: 4 }), Some(&3));
        assert_eq!(space_image.img.get(&Position { x: 1, y: 5 }), Some(&4));
        assert_eq!(space_image.img.get(&Position { x: 9, y: 6 }), Some(&5));
        assert_eq!(space_image.img.get(&Position { x: 7, y: 8 }), Some(&6));
        assert_eq!(space_image.img.get(&Position { x: 0, y: 9 }), Some(&7));
        assert_eq!(space_image.img.get(&Position { x: 4, y: 9 }), Some(&8));

    }

    #[test]
    fn test_get_empty() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let space_image = SpaceImage::from_string(input);
        let empty_columns = space_image.get_empty_columns();
        let empty_rows = space_image.get_empty_rows();
        assert_eq!(empty_columns.len(), 3);
        assert_eq!(empty_rows.len(), 2);
        assert!(empty_columns.contains(&2));
        assert!(empty_columns.contains(&5));
        assert!(empty_columns.contains(&8));
        assert!(empty_rows.contains(&3));
        assert!(empty_rows.contains(&7));
    }
}