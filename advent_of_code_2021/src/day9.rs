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
    let low_points = find_low_points(&map);

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

fn find_low_points(heightmap: &[Vec<u8>]) -> Vec<Point> {
    let mut low_points: Vec<Point> = Vec::new();
    let neighbors: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    for (row_idx, subvec) in heightmap.iter().enumerate() {
        for (col_idx, value) in subvec.iter().enumerate() {
            let is_lower: bool = neighbors
                .iter()
                .filter(|&neighbor| {
                    point_on_map(
                        heightmap,
                        row_idx as i32 + neighbor.0,
                        col_idx as i32 + neighbor.1,
                    )
                })
                .all(|&neighbor| {
                    *value
                        < heightmap[(row_idx as i32 + neighbor.0) as usize]
                            [(col_idx as i32 + neighbor.1) as usize]
                });

            if is_lower {
                low_points.push(Point {
                    value: *value,
                    row: row_idx,
                    col: col_idx,
                })
            }
        }
    }

    low_points
}

fn point_on_map<T>(heightmap: &[Vec<T>], row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    }
    if let Some(subvec) = heightmap.get(row as usize) {
        if let Some(_item) = subvec.get(col as usize) {
            return true;
        }
    }

    false
}

/// # --- Part Two ---
/// Next, you need to find the largest basins so you know what areas are most important to avoid.
///
/// A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.
///
/// The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.
///
/// The top-left basin, size 3:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
/// The top-right basin, size 9:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
/// The middle basin, size 14:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
/// The bottom-right basin, size 9:
///
/// 2199943210
/// 3987894921
/// 9856789892
/// 8767896789
/// 9899965678
/// Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.
///
/// What do you get if you multiply together the sizes of the three largest basins?
pub fn part2() {
    let data = read_from_data_dir("day9.txt").unwrap();
    let map = string_to_map(data);
    let low_points = find_low_points(&map);
    let member_map = convert_map_from_int_to_members(map);
    let filled_map = fill_basins(member_map, low_points);
    let mut basin_counts = count_basins(&filled_map);
    basin_counts.sort_unstable();
    basin_counts.reverse();

    println!(
        "Day9:2 The three largest basins multiplied together are {}",
        basin_counts.iter().take(3).fold(1, |acc, x| acc * x)
    );
}

fn convert_map_from_int_to_members(heightmap: Vec<Vec<u8>>) -> Vec<Vec<BasinMember>> {
    let mut member_map = vec![
        vec![
            BasinMember {
                basin_id: 0,
                claimed: false,
                value: 0
            };
            heightmap[0].len()
        ];
        heightmap.len()
    ];

    for (r_idx, row) in heightmap.iter().enumerate() {
        for (c_idx, value) in row.iter().enumerate() {
            member_map[r_idx][c_idx].value = *value;
        }
    }

    member_map
}

#[derive(Debug, Copy, Clone)]
struct BasinMember {
    basin_id: u32,
    claimed: bool,
    value: u8,
}

fn fill_basins(mut map: Vec<Vec<BasinMember>>, low_points: Vec<Point>) -> Vec<Vec<BasinMember>> {
    // create starting points
    for (idx, low_point) in low_points.iter().enumerate() {
        map[low_point.row][low_point.col].claimed = true;
        map[low_point.row][low_point.col].basin_id = idx as u32 + 1;
    }

    let mut all_filled = false;
    let neighbors: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];

    while !all_filled {
        for row in 0..map.len() {
            for col in 0..map[0].len() {
                if map[row][col].value == 9 {
                    continue;
                } else {
                    if !map[row][col].claimed {
                        // check if any neighbors are in a basin already

                        for loc in neighbors.iter() {
                            let mut neighbor_holder: Option<BasinMember> = None;
                            if let Some(subvec) = map.get((loc.0 + row as i32) as usize) {
                                if let Some(neighbor) = subvec.get((loc.1 + col as i32) as usize) {
                                    if neighbor.claimed {
                                        neighbor_holder = Some(*neighbor);
                                    }
                                }
                            }
                            // cannot assign to the map while in the mutable for loop above, so I have to hold a reference to the nieghbor here
                            if let Some(holder) = neighbor_holder {
                                map[row][col].claimed = true;
                                map[row][col].basin_id = holder.basin_id;
                                continue;
                            }
                        }
                    }
                }
            }
        }

        // check if all filled
        all_filled = true;
        for row in 0..map.len() {
            for col in 0..map[0].len() {
                if map[row][col].value != 9 && !map[row][col].claimed {
                    all_filled = false;
                    continue;
                }
            }
        }
    }
    for row in map.iter() {
        for item in row.iter() {
            print!("{} ", item.basin_id);
        }
        println!();
    }
    map
}

fn count_basins(map: &[Vec<BasinMember>]) -> Vec<i32> {
    let member_occurences: Vec<u32> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|member| member.basin_id)
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect::<Vec<u32>>();
    let maximum = member_occurences.iter().max().unwrap();
    let mut bins = vec![0; *maximum as usize + 1];
    member_occurences.iter().for_each(|&occ| {
        bins[occ as usize] += 1;
    });

    // removing index 0 which is the 9's
    bins.remove(0);
    bins
}

pub fn is_complete() -> bool {
    true
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

        let low_points = find_low_points(&data);

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
    fn test_convert_to_member_map() {
        let map = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        println!("map: {:?}", convert_map_from_int_to_members(map));
    }

    #[test]
    fn test_count_bins() {
        let map = vec![
            vec![
                BasinMember {
                    basin_id: 1,
                    claimed: true,
                    value: 5,
                },
                BasinMember {
                    basin_id: 1,
                    claimed: true,
                    value: 6,
                },
            ],
            vec![
                BasinMember {
                    basin_id: 1,
                    claimed: true,
                    value: 7,
                },
                BasinMember {
                    basin_id: 2,
                    claimed: true,
                    value: 8,
                },
            ],
        ];

        assert_eq!(count_basins(&map), vec![0, 3, 1])
    }
}
