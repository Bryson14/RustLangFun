use crate::read_from_data_dir;

/// --- Day 5: Hydrothermal Venture ---
/// You come across a field of hydrothermal vents on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to avoid them if possible.
///
/// They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents (your puzzle input) for you to review. For example:
/// ```text
/// 0,9 -> 5,9
/// 8,0 -> 0,8
/// 9,4 -> 3,4
/// 2,2 -> 2,1
/// 7,0 -> 7,4
/// 6,4 -> 2,0
/// 0,9 -> 2,9
/// 3,4 -> 1,4
/// 0,0 -> 8,8
/// 5,5 -> 8,2
/// ```
/// Each line of vents is given as a line segment in the format x1,y1 -> x2,y2 where x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end. These line segments include the points at both ends. In other words:
///
/// An entry like 1,1 -> 1,3 covers points 1,1, 1,2, and 1,3.
/// An entry like 9,7 -> 7,7 covers points 9,7, 8,7, and 7,7.
/// For now, only consider horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
///
/// So, the horizontal and vertical lines from the above list would produce the following diagram:
/// ```text
/// .......1..
/// ..1....1..
/// ..1....1..
/// .......1..
/// .112111211
/// ..........
/// ..........
/// ..........
/// ..........
/// 222111....
/// ```
/// In this diagram, the top left corner is 0,0 and the bottom right corner is 9,9. Each position is shown as the number of lines which cover that point or . if no line covers that point. The top-left pair of 1s, for example, comes from 2,2 -> 2,1; the very bottom row is formed by the overlapping lines 0,9 -> 5,9 and 0,9 -> 2,9.
///
/// To avoid the most dangerous areas, you need to determine the number of points where at least two lines overlap. In the above example, this is anywhere in the diagram with a 2 or larger - a total of 5 points.
///
/// Consider only horizontal and vertical lines. At how many points do at least two lines overlap?
pub fn part1() {
    let data = read_from_data_dir("day5.txt").unwrap();
    let points = input_data_to_points(data);
    let no_diagonal = true;
    let danger_zones = find_dangerous_area(&points, 2, no_diagonal);
    println!(
        "Day5:1. There are {} points of overlapping lines",
        danger_zones
    );
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

/// finds the numbers to area (points on the map) where level is or over the
/// danger threshold
fn find_dangerous_area(points: &[(Point, Point)], danger_threshold: u32, no_diagonal: bool) -> u32 {
    let map = create_map(points, no_diagonal);
    count_danger_areas(map, danger_threshold)
}

/// helper function to found the numbers of spots on the map that are crosses of two lines
fn count_danger_areas(map: Vec<Vec<u32>>, danger_threshold: u32) -> u32 {
    map.iter()
        .map(|row| row.iter().filter(|&&num| num >= danger_threshold).count() as u32)
        .sum()
}

/// helper function that creates the map based on the input points
fn create_map(points: &[(Point, Point)], no_diagonal: bool) -> Vec<Vec<u32>> {
    let max_height = points
        .iter()
        .map(|(p1, p2)| if p1.y > p2.y { p1.y } else { p2.y })
        .max()
        .unwrap();
    let max_width = points
        .iter()
        .map(|(p1, p2)| if p1.x > p2.x { p1.x } else { p2.x })
        .max()
        .unwrap();

    let mut map: Vec<Vec<u32>> = vec![vec![0; max_width + 1]; max_height + 1];

    points.iter().for_each(|point_pair| {
        let all_points = get_line_between_points(point_pair, no_diagonal);
        all_points
            .iter()
            .for_each(|point| map[point.y][point.x] += 1);
    });
    map
}

/// helper function. Given two points, like 0,0 and 0,3, it returns
/// a vector of all the points inbetween [(0,0),(0,1),(0,2),(0,3)]
/// Assumes vertical or horizontal line
fn get_line_between_points(point_pair: &(Point, Point), no_diagonal: bool) -> Vec<Point> {
    let (start, end) = point_pair;
    let mut line = Vec::new();
    if start.x == end.x {
        if start.y < end.y {
            for y in start.y..=end.y {
                let p = Point { x: start.x, y };
                line.push(p);
            }
        } else {
            for y in end.y..=start.y {
                let p = Point { x: start.x, y };
                line.push(p);
            }
        }
    } else if start.y == end.y {
        if start.x < end.x {
            for x in start.x..=end.x {
                let p = Point { x, y: start.y };
                line.push(p);
            }
        } else {
            for x in end.x..=start.x {
                let p = Point { x, y: start.y };
                line.push(p);
            }
        }
    // diagonal lines. assumes only 45 degree angles
    } else if !no_diagonal {
        // left to right down line
        if start.x < end.x && start.y < end.y {
            let dist = end.x - start.x;
            for i in 0..=dist {
                let p = Point {
                    x: start.x + i,
                    y: start.y + i,
                };
                line.push(p);
            }

            // left to right up line
        } else if start.x < end.x && start.y > end.y {
            let dist = end.x - start.x;
            for i in 0..=dist {
                let p = Point {
                    x: start.x + i,
                    y: start.y - i,
                };
                line.push(p);
            }

            // right to left down line
        } else if start.x > end.x && start.y < end.y {
            let dist = start.x - end.x;
            for i in 0..=dist {
                let p = Point {
                    x: start.x - i,
                    y: start.y + i,
                };
                line.push(p);
            }

            // right to left up line
        } else if start.x > end.x && start.y > end.y {
            let dist = start.x - end.x;
            for i in 0..=dist {
                let p = Point {
                    x: end.x + i,
                    y: end.y + i,
                };
                line.push(p);
            }

        // bad news bears
        } else {
            unreachable!();
        }
    }
    line
}

fn input_data_to_points(s: String) -> Vec<(Point, Point)> {
    s.lines()
        .map(|line| {
            let mut line_iter = line.split(" -> ");
            let segment1 = line_iter.next().unwrap();
            let mut iter = segment1.trim().split(',');
            let x1 = iter.next().unwrap().parse::<usize>().unwrap();
            let y1 = iter.next().unwrap().parse::<usize>().unwrap();

            let segment2 = line_iter.next().unwrap();
            let mut iter = segment2.trim().split(',');
            let x2 = iter.next().unwrap().parse::<usize>().unwrap();
            let y2 = iter.next().unwrap().parse::<usize>().unwrap();
            (Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
        })
        .collect::<Vec<(Point, Point)>>()
}

pub fn part2() {
    let data = read_from_data_dir("day5.txt").unwrap();
    let points = input_data_to_points(data);
    let no_diagonal = false;
    let danger_zones = find_dangerous_area(&points, 2, no_diagonal);
    println!(
        "Day5:2. There are {} points of overlapping lines with diagonal lines",
        danger_zones
    );
}

pub fn is_complete() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_data_to_points() {
        let points = vec![(Point { x: 0, y: 9 }, Point { x: 5, y: 9 })];
        let input = input_data_to_points("0,9 -> 5,9\n".into());

        assert_eq!(input[0], points[0]);
    }

    #[test]
    fn test_count_danger_areas() {
        let map = vec![vec![1, 0], vec![2, 2]];
        assert_eq!(count_danger_areas(map, 2), 2);
    }

    #[test]
    fn test_get_line_between_points() {
        let points = (Point { x: 0, y: 9 }, Point { x: 3, y: 9 });
        let expected = vec![
            Point { x: 0, y: 9 },
            Point { x: 1, y: 9 },
            Point { x: 2, y: 9 },
            Point { x: 3, y: 9 },
        ];
        assert_eq!(get_line_between_points(&points, true), expected);
    }

    #[test]
    fn test_get_line_between_points_2() {
        let points = (Point { x: 2, y: 2 }, Point { x: 2, y: 1 });
        let expected = vec![Point { x: 2, y: 1 }, Point { x: 2, y: 2 }];
        assert_eq!(get_line_between_points(&points, true), expected);
    }

    #[test]
    fn test_get_line_between_points_3() {
        let points = (Point { x: 1, y: 1 }, Point { x: 3, y: 3 });
        let expected = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];
        assert_eq!(get_line_between_points(&points, false), expected);
    }

    #[test]
    fn test_get_line_between_points_4() {
        let points = (Point { x: 3, y: 3 }, Point { x: 1, y: 1 });
        let expected = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];
        assert_eq!(get_line_between_points(&points, false), expected);
    }

    #[test]
    fn test_get_line_between_points_5() {
        let points = (Point { x: 3, y: 1 }, Point { x: 1, y: 3 });
        let expected = vec![
            Point { x: 3, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 1, y: 3 },
        ];
        assert_eq!(get_line_between_points(&points, false), expected);
    }

    #[test]
    fn test_get_line_between_points_6() {
        let points = (Point { x: 1, y: 3 }, Point { x: 3, y: 1 });
        let expected = vec![
            Point { x: 1, y: 3 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 1 },
        ];
        assert_eq!(get_line_between_points(&points, false), expected);
    }

    #[test]
    fn test_create_map() {
        let points = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2".into();
        let points = input_data_to_points(points);
        let expected_map = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(create_map(&points, true), expected_map);
    }

    #[test]
    fn test_create_map_2() {
        let points = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2".into();
        let points = input_data_to_points(points);

        let expected_map = vec![
            vec![1, 0, 1, 0, 0, 0, 0, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 0, 0, 2, 0, 0],
            vec![0, 0, 2, 0, 1, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 2, 0, 2, 0, 0],
            vec![0, 1, 1, 2, 3, 1, 3, 2, 1, 1],
            vec![0, 0, 0, 1, 0, 2, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(create_map(&points, false), expected_map);
    }
}
