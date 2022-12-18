#![allow(unused)]
use crate::utils::read_data;
use std::{collections::HashSet, future::poll_fn};

const FILE: &str = "day18.txt";
const DAY: &str = "{{ DAY 18 }}";

/// --- Day 18: Boiling Boulders ---
/// You and the elephants finally reach fresh air. You've emerged near the base of a large volcano that
/// seems to be actively erupting!
/// Fortunately, the lava seems to be flowing away from you and toward the ocean.
pub fn part1() {
    let data = read_data(FILE);
    let cube_positions: Vec<Cube> = read_cube_positions(data);
    let ans = sum_surface_area(cube_positions);
    println!("{DAY}-1 Total surface area of droplets is {ans}");
}

pub fn part2() {
    let data = read_data(FILE);
    let cube_positions: Vec<Cube> = read_cube_positions(data);
    let ans = sum_exterior_surface_area(cube_positions);
    println!("{DAY}-2 Total exterior surface area of droplets is {ans}");
}

fn read_cube_positions(data: String) -> Vec<Cube> {
    data.lines()
        .map(|line| {
            Cube::from_vec(
                line.split(',')
                    .map(|c| c.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect()
}

fn find_air_cubes(cubes: &Vec<Cube>) -> HashSet<Cube> {
    let mut cubes_set: HashSet<&Cube> = HashSet::from_iter(cubes.iter());
    let mut possible_pockets = Vec::with_capacity(cubes.len() * 6);
    for cube in cubes.iter() {
        let x_plus = Cube {
            x: cube.x + 1,
            y: cube.y,
            z: cube.z,
        };
        if !cubes_set.contains(&x_plus) {
            possible_pockets.push(x_plus);
        }
        if cube.x > 0 {
            let x_minus = Cube {
                x: cube.x - 1,
                y: cube.y,
                z: cube.z,
            };
            if !cubes_set.contains(&x_minus) {
                possible_pockets.push(x_minus);
            }
        }
        let y_plus = Cube {
            x: cube.x,
            y: cube.y + 1,
            z: cube.z,
        };
        if !cubes_set.contains(&y_plus) {
            possible_pockets.push(y_plus);
        }
        if cube.y > 0 {
            let y_minus = Cube {
                x: cube.x,
                y: cube.y - 1,
                z: cube.z,
            };
            if !cubes_set.contains(&y_minus) {
                possible_pockets.push(y_minus);
            }
        }
        let z_plus = Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z + 1,
        };
        if !cubes_set.contains(&z_plus) {
            possible_pockets.push(z_plus);
        }
        if cube.z > 0 {
            let z_minus = Cube {
                x: cube.x,
                y: cube.y,
                z: cube.z - 1,
            };
            if !cubes_set.contains(&z_minus) {
                possible_pockets.push(z_minus);
            }
        }
    }

    let mut i = 0;
    while i < possible_pockets.len() {
        let mut neighbors = 0;
        for cube in cubes {
            if possible_pockets[i].are_neighbors(cube) {
                neighbors += 1;
            }
        }

        if neighbors == 6 {
            // is a air pocket
            i += 1;
        } else if neighbors > 6 {
            unreachable!("Cant have more than 6 orthogonal neighbors");
        } else {
            possible_pockets.remove(i);
        }
    }
    HashSet::from_iter(possible_pockets.into_iter())
}

fn sum_exterior_surface_area(cubes: Vec<Cube>) -> usize {
    // for each cube, if it has neighbor in one of the 6 directions, then remove a surface area unit
    let mut sum_area = 0;
    let air_pockets = find_air_cubes(&cubes);
    println!("Pockets");
    for p in air_pockets.iter() {
        println!("{p:?}");
    }
    for i in 0..cubes.len() {
        let mut neighbors = 0;
        let mut pocket_neighbors = 0;
        for j in 0..cubes.len() {
            if i == j {
                continue;
            }

            if cubes[i].are_neighbors(&cubes[j]) {
                neighbors += 1;
            }
        }

        // subtract air pockets too from overall surface area
        for pocket in air_pockets.iter() {
            if cubes[i].are_neighbors(pocket) {
                pocket_neighbors += 1;
            }
        }

        if neighbors + pocket_neighbors > 6 {
            unreachable!("Wrong calculation. Cube has more than 6 orthoganal neighbors!")
        }
        sum_area += 6 - neighbors - pocket_neighbors;
    }
    sum_area
}

fn sum_surface_area(cubes: Vec<Cube>) -> usize {
    // for each cube, if it has neighbor in one of the 6 directions, then remove a surface area unit
    let mut sum_area = 0;
    for i in 0..cubes.len() {
        let mut neighbors = 0;
        for j in 0..cubes.len() {
            if i == j {
                continue;
            }

            if cubes[i].are_neighbors(&cubes[j]) {
                neighbors += 1;
            }
        }

        if neighbors > 6 {
            unreachable!("Wrong calculation. Cube has more than 6 orthoganal neighbors!")
        }
        sum_area += 6 - neighbors;
    }
    sum_area
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn from_vec(positions: Vec<usize>) -> Self {
        assert_eq!(positions.len(), 3);

        Cube {
            x: positions[0],
            y: positions[1],
            z: positions[2],
        }
    }

    fn are_neighbors(&self, other: &Cube) -> bool {
        // diff Z
        if self.x == other.x && self.y == other.y && usize_1_diff(self.z, other.z) {
            return true;
        }
        // diff Y
        if self.x == other.x && usize_1_diff(self.y, other.y) && self.z == other.z {
            return true;
        }
        // diff X
        if usize_1_diff(self.x, other.x) && self.y == other.y && self.z == other.z {
            return true;
        }
        false
    }
}

fn usize_1_diff(l: usize, r: usize) -> bool {
    ((l as i64) - (r as i64)).abs() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_air_pockets() {
        let data = "2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5";
        let cubes = read_cube_positions(data.into());
        let pockets = find_air_cubes(&cubes);
        let pockets = Vec::from_iter(pockets.into_iter());
        assert_eq!(pockets[0], Cube { x: 2, y: 2, z: 5 });
        assert_eq!(pockets.len(), 1);
        println!("{pockets:?}");
    }

    #[test]
    fn test_find_air_pockets_2() {
        let data = "2,2,2
        0,2,2
        1,3,2
        1,1,2
        1,2,1
        1,2,3";
        /// 1,2,2
        let cubes = read_cube_positions(data.into());
        let pockets = find_air_cubes(&cubes);
        let pockets = Vec::from_iter(pockets.into_iter());
        assert_eq!(pockets[0], Cube { x: 1, y: 2, z: 2 });
        assert_eq!(pockets.len(), 1);
        println!("{pockets:?}");
    }

    #[test]
    fn test_find_air_pockets_3() {
        let data = "2,2,2
        0,2,2
        1,3,2
        1,1,2
        1,2,1
        1,2,3
        2,3,3
        4,3,3
        3,4,3
        3,2,3
        3,3,2
        3,3,4";
        /// 3,3,3
        let cubes = read_cube_positions(data.into());
        let pockets = find_air_cubes(&cubes);
        let pockets = Vec::from_iter(pockets.into_iter());
        assert_eq!(pockets[1], Cube { x: 3, y: 3, z: 3 });
        assert_eq!(pockets[0], Cube { x: 1, y: 2, z: 2 });
        assert_eq!(pockets.len(), 2);
        println!("{pockets:?}");
    }
}
