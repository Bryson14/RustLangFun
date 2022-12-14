#![allow(unused)]
use crate::utils::read_data;

const FILE: &str = "day12.txt";
const DAY: &str = "{{ DAY 12 }}";

/// --- Day 12: Hill Climbing Algorithm ---
pub fn part1() {
    let data = read_data(FILE);
    let map = read_map(data);
    let current = vec![map.start];
    let path = find_shortest_path(&map, current);
    println!("{DAY}-1 Path len is {}", path.unwrap().len());
}

pub fn part2() {
    let data = read_data(FILE);
}

fn find_shortest_path(
    map: &ElevationMap,
    path: Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    const MOVES: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    if path.is_empty() {
        unreachable!()
    }
    let current = path.last().unwrap();
    if current.0 == map.end.0 && current.1 == map.end.1 {
        return Some(path);
    }
    let mut shortest: Option<Vec<(usize, usize)>> = None;
    for poss_move in MOVES {
        if in_bounds(map, current, &poss_move)
            && is_climbable(map, current, &poss_move)
            && !already_visited(&path, current, &poss_move)
        {
            let mut new_path = path.clone();
            new_path.push(add_path(current, &poss_move));
            let found = find_shortest_path(map, new_path);
            let shortest_clone = shortest.clone();
            match shortest_clone {
                None => shortest = found,
                Some(position_path) => {
                    let found = found.unwrap();
                    if position_path.len() > found.len() {
                        shortest = Some(found);
                    }
                }
            }
        }
    }

    shortest
}

fn in_bounds(map: &ElevationMap, curr: &(usize, usize), poss_move: &(i32, i32)) -> bool {
    let x = curr.0 as i32 + poss_move.0;
    let y = curr.1 as i32 + poss_move.1;
    x >= 0 && y >= 0 && y < map.map.len() as i32 && x < map.map[0].len() as i32
}

fn is_climbable(map: &ElevationMap, curr: &(usize, usize), poss_move: &(i32, i32)) -> bool {
    const LARGEST_ELEVATION_DIFF: i32 = 1;
    let x = (curr.0 as i32 + poss_move.0) as usize;
    let y = (curr.1 as i32 + poss_move.1) as usize;
    (map.map[y][x] as i32 - map.map[curr.1][curr.0] as i32).abs() <= LARGEST_ELEVATION_DIFF
}

fn add_path(curr: &(usize, usize), poss_move: &(i32, i32)) -> (usize, usize) {
    let x = (curr.0 as i32 + poss_move.0) as usize;
    let y = (curr.1 as i32 + poss_move.1) as usize;
    (x, y)
}

fn already_visited(
    path: &Vec<(usize, usize)>,
    curr: &(usize, usize),
    poss_move: &(i32, i32),
) -> bool {
    let x = (curr.0 as i32 + poss_move.0) as usize;
    let y = (curr.1 as i32 + poss_move.1) as usize;
    path.contains(&(x, y))
}

#[derive(Debug, PartialEq)]
struct ElevationMap {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<u8>>,
}

fn read_map(data: String) -> ElevationMap {
    const START_MARKER: u8 = 99;
    const END_MARKER: u8 = 100;
    let mut map: Vec<Vec<u8>> = data
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| {
                    if c == 'E' {
                        END_MARKER
                    } else if c == 'S' {
                        START_MARKER
                    } else {
                        c as u8 - 'a' as u8
                    }
                })
                .collect()
        })
        .collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in map.iter_mut().enumerate() {
        for (x, val) in row.iter_mut().enumerate() {
            if val == &mut START_MARKER {
                start = (x, y);
                *val = 'a' as u8 - 'a' as u8
            } else if val == &mut END_MARKER {
                end = (x, y);
                *val = 'z' as u8 - 'a' as u8
            }
        }
    }

    ElevationMap { start, end, map }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_map() {
        let data = "Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi";

        let map = read_map(data.into());
        println!("{:?}", map);
        assert_eq!(
            map,
            ElevationMap {
                start: (0, 0),
                end: (5, 2),
                map: vec![
                    vec![0, 0, 1, 16, 15, 14, 13, 12],
                    vec![0, 1, 2, 17, 24, 23, 23, 11],
                    vec![0, 2, 2, 18, 25, 25, 23, 10],
                    vec![0, 2, 2, 19, 20, 21, 22, 9],
                    vec![0, 1, 3, 4, 5, 6, 7, 8]
                ]
            }
        )
    }
}
