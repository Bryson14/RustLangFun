#![allow(unused)]
use std::{collections::HashMap, fmt::Display};

use crate::utils::read_data;

const FILE: &str = "day14.txt";
const DAY: &str = "{{ DAY 14 }}";

/// --- Day 14: Regolith Reservoir ---
pub fn part1() {
    let data = read_data(FILE);
    let mut sandmap = read_map(data);
    sandmap.drop_sand();
    let ans = sandmap.count_sand();

    println!("{DAY} the number of sand blocks is {ans}");
}

pub fn part2() {
    let data = read_data(FILE);
}

fn make_rock_lines(map: &mut HashMap<(usize, usize), Material>, vertices: &Vec<(usize, usize)>) {
    let mut point1 = 0;
    let mut point2 = 1;
    assert!(vertices.len() >= 2);

    loop {
        if point2 >= vertices.len() {
            break;
        }
        let p1 = vertices[point1];
        let p2 = vertices[point2];

        // make a vertical line
        if p1.0 == p2.0 {
            let mut ys = [p1.1, p2.1];
            ys.sort();
            for y in (ys[0]..=ys[1]) {
                map.insert((p1.0, y), Material::Rock);
            }

        // horizontal line
        } else if p1.1 == p2.1 {
            let mut xs = [p1.0, p2.0];
            xs.sort();
            for x in (xs[0]..=xs[1]) {
                map.insert((x, p1.1), Material::Rock);
            }
        } else {
            panic!("Cannot make a diagonal line");
        }

        point1 += 1;
        point2 += 1;
    }
}

fn read_map(data: String) -> SandMap {
    let mut hashmap: HashMap<(usize, usize), Material> = HashMap::new();

    let vertices_list = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(|pos| pos.trim())
                .map(|pos| {
                    let (l, r) = pos.split_once(',').unwrap();
                    let l = l.parse::<usize>().unwrap();
                    let r = r.parse::<usize>().unwrap();
                    (l, r)
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    for vertices in vertices_list {
        make_rock_lines(&mut hashmap, &vertices)
    }

    let minx = hashmap.iter().map(|((x, _), _)| x).min().unwrap();
    let maxx = hashmap.iter().map(|((x, _), _)| x).max().unwrap();
    let miny = &0;
    let maxy = hashmap.iter().map(|((_, y), _)| y).max().unwrap();
    let x_diff = maxx - minx + 1;
    let y_diff = maxy - miny + 1;
    let mut map: Vec<Vec<Material>> = vec![vec![Material::Air; x_diff]; y_diff];

    hashmap.iter().for_each(|(&(x, y), &mat)| {
        let x_shift = x - minx;
        let y_shift = y - miny;
        map[y_shift][x_shift] = mat;
    });

    SandMap {
        map,
        xrange: (*minx, *maxx),
        yrange: (*miny, *maxy),
    }
}

struct SandMap {
    map: Vec<Vec<Material>>,
    yrange: (usize, usize),
    xrange: (usize, usize),
}

impl SandMap {
    fn drop_sand(&mut self) {
        let mut keep_dropping = true;
        while (keep_dropping) {
            keep_dropping = self.drop_sand_block();
        }
    }

    fn drop_sand_block(&mut self) -> bool {
        let sand_origin: (usize, usize) = (500, 0);
        let mut pos = (sand_origin.0 - self.xrange.0, sand_origin.1 - self.yrange.0);
        let mut falling = true;

        while self.inbounds(pos) && falling {
            if self.map[pos.1 + 1][pos.0] == Material::Air {
                // move down
                pos.1 += 1;
            } else if pos.0 > 0 && self.map[pos.1 + 1][pos.0 - 1] == Material::Air {
                // move down and left
                pos.0 -= 1;
                pos.1 += 1;
            } else if pos.0 < self.xrange.0 && self.map[pos.1 + 1][pos.0 + 1] == Material::Air {
                // move down and right
                pos.0 += 1;
                pos.1 += 1;
            } else if self.map[pos.1 + 1][pos.0] == Material::Rock {
                falling = false;
            }
        }

        if !self.inbounds(pos) {
            // falling sand
            false
        } else {
            self.map[pos.1][pos.0] = Material::Sand;
            true
        }
    }

    fn inbounds(&self, pos: (usize, usize)) -> bool {
        if let Some(row) = self.map.get(pos.1) {
            if let Some(v) = row.get(pos.0) {
                return true;
            }
        }
        false
    }

    fn count_sand(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().map(|m| m == &Material::Sand).count())
            .sum()
    }
}

impl std::fmt::Display for SandMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self
            .map
            .iter()
            .map(|r| {
                r.iter()
                    .map(|m| format!("{m}"))
                    .fold(String::new(), |a, b| a + &b)
            })
            .fold(String::new(), |a, b| a + &b + "\n");

        // let metadata = format!("X Range: {:?} | Y Range: {:?}\n", self.xrange, self.yrange);
        // s.insert_str(0, &metadata);
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Material {
    Sand,
    Rock,
    Air,
    FlowingSand,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Material::FlowingSand => '~',
            Material::Air => '.',
            Material::Rock => '#',
            Material::Sand => 'o',
        };
        write!(f, "{c}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let print_map = "..........\n..........\n..........\n..........\n....#...##\n....#...#.\n..###...#.\n........#.\n........#.\n#########.\n";
        let data = "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9"
            .into();
        let sandmap = read_map(data);
        println!("{sandmap}");
        assert_eq!(format!("{sandmap}"), print_map);
    }

    #[test]
    fn test_drop_sand() {
        let print_map = "..........\n..........\n..........\n..........\n....#...##\n....#...#.\n..###...#.\n........#.\n........#.\n#########.\n";
        let data = "498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9"
            .into();
        let mut sandmap = read_map(data);
        sandmap.drop_sand();
        let ans = sandmap.count_sand();
        assert!(ans == 24);
    }
}
