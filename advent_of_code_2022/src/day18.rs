#![allow(unused)]
use crate::utils::read_data;

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

fn find_air_cubes(cubes: &Vec<Cube>) -> Vec<Cube> {
    todo!()
}

fn sum_exterior_surface_area(cubes: Vec<Cube>) -> usize {
    // for each cube, if it has neighbor in one of the 6 directions, then remove a surface area unit
    let mut sum_area = 0;
    let air_pockets = find_air_cubes(&cubes);
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

        // subtract air pockets too
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
    fn test() {}
}
