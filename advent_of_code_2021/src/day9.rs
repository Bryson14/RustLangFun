use crate::read_from_data_dir;

/// # --- Day 9: Smoke Basin ---
/// These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.
///
/// If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).
///
/// Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:
/// ```text
/// 2*1 9 9 9 4 3 2 1*0
/// 3 9 8 7 8 9 4 9 2 1
/// 9 8*5 6 7 8 9 8 9 2
/// 8 7 6 7 8 9 6 7 8 9
/// 9 8 9 9 9 6*5 6 7 8
/// ```
/// Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.
///
/// Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)
///
/// In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.
///
/// The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.
///
/// Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?
pub fn part1() {
    let data = read_from_data_dir("day9.txt").unwrap();
    let map = string_to_map(data);
    let ans = sum_low_points(map);
    println!("Day9:1 The risk level sum for all low points is {}", ans);
}

fn string_to_map(s: String) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn sum_low_points(map: Vec<Vec<u8>>) -> u64 {
    let low_points = find_low_points(map);
    // println!("low points: {:?}", low_points);

    low_points
        .iter()
        .map(|point| point.get_risk_level() as u64)
        .sum()
}

#[derive(Debug, PartialEq)]
struct Point {
    value: u8,
    row: usize,
    col: usize,
}

impl Point {
    fn get_risk_level(&self) -> u8 {
        self.value + 1
    }
}

fn find_low_points(heightmap: Vec<Vec<u8>>) -> Vec<Point> {
    let mut low_points: Vec<Point> = Vec::new();

    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            // checking above (row - 1)
            if row > 0 && heightmap[row][col] > heightmap[row - 1][col] {
                continue;
            }

            // checking below (row + 1)
            if row < heightmap.len() - 1 && heightmap[row][col] > heightmap[row + 1][col] {
                continue;
            }

            // checking left (col - 1)
            if col > 0 && heightmap[row][col] > heightmap[row][col - 1] {
                continue;
            }

            // checking right (col + 1)
            if col < heightmap[row].len() - 1 && heightmap[row][col] > heightmap[row][col + 1] {
                continue;
            }

            // if passed all the conditions, then adds the low point.
            low_points.push(Point {
                value: heightmap[row][col],
                row,
                col,
            });
        }
    }

    low_points
}

pub fn part2() {}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_low_points() {
        let data: Vec<Vec<u8>> = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        let low_points = find_low_points(data);
        println!("low points: {:?}", low_points);

        assert!(low_points.contains(&Point {
            value: 1u8,
            row: 0,
            col: 1
        }));
        assert!(low_points.contains(&Point {
            value: 0u8,
            row: 0,
            col: 9
        }));
        assert!(low_points.contains(&Point {
            value: 5u8,
            row: 2,
            col: 2
        }));
        assert!(low_points.contains(&Point {
            value: 5u8,
            row: 4,
            col: 6
        }));
    }

    #[test]
    fn test_sum_low_points() {
        let data: Vec<Vec<u8>> = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];

        assert_eq!(sum_low_points(data), 15);
    }

    #[test]
    fn test_movement_with_aim() {}
}
