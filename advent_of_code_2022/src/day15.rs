#![allow(unused)]
use crate::utils::read_data;
use rayon::prelude::*;
use regex::Regex;

const FILE: &str = "day15.txt";
const DAY: &str = "{{ DAY 15 }}";

/// --- Day 15: Beacon Exclusion Zone ---
/// Consult the report from the sensors you just deployed. In the row where y=2000000,
/// how many positions cannot contain a beacon?
pub fn part1() {
    let data = read_data(FILE);
    let sensors = read_sensor_data(data);
    let row = 2000000;
    let ans = cannot_contain_beacon(sensors, row);
    println!("{DAY}-1 Row {row} has {ans} positions where the beacon cannot be");
}

/// --- Part Two ---
/// Your handheld device indicates that the distress signal is coming from a beacon nearby.
/// The distress beacon is not detected by any sensor, but the distress beacon must have
///  x and y coordinates each no lower than 0 and no larger than 4000000.
///
/// To isolate the distress beacon's signal, you need to determine its tuning frequency,
/// which can be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.
///
/// In the example above, the search space is smaller: instead, the x and y coordinates
/// can each be at most 20. With this reduced search area, there is only a single position
/// that could have a beacon: x=14, y=11. The tuning frequency for this distress beacon is 56000011.
///
/// Find the only possible position for the distress beacon. What is its tuning frequency?
pub fn part2() {
    let data = read_data(FILE);
    let sensors = read_sensor_data(data);
    let search_space = GridRange {
        x_range: (0, 4000000),
        y_range: (0, 4000000),
    };
    let ans = find_no_beacon_zone(sensors, search_space);
    println!(
        "{DAY}-2 Tuning freq of missing beacon is {} at x:{} and y:{}",
        ans.get_tuning_freqency(),
        ans.x,
        ans.y
    );
}

fn find_no_beacon_zone(sensors: Vec<Sensor>, search_space: GridRange) -> GridPos {
    let mut map_range: GridRange = GridRange {
        x_range: (0, 0),
        y_range: (0, 0),
    };
    sensors
        .iter()
        .for_each(|s| map_range.expand_range(s.get_largest_range()));
    let beacon_pos = sensors
        .iter()
        .map(|s| s.closest_beacon.pos)
        .collect::<Vec<GridPos>>();

    let mut uncovered_point = GridPos { x: 0, y: 0 };
    let mut point = GridPos { x: 0, y: 0 };

    let _ = (search_space.x_range.0..search_space.x_range.1)
        .into_par_iter()
        .find_first(|&x| {
            let mut point = GridPos { x: 0, y: 0 };
            for y in (search_space.y_range.0..=search_space.y_range.1) {
                (point.x, point.y) = (x, y);
                let not_covered = sensors.iter().find(|s| !s.covers_point(&point));
                if not_covered.is_some() {}
            }
            uncovered_point = point;
            true
        })
        .unwrap();
    uncovered_point
}

/// So, suppose you have an arrangement of beacons and sensors like in the example above and,
/// just in the row where y=10, you'd like to count the number of positions a beacon cannot possibly exist.
/// The coverage from all sensors near that row looks like this:
///
/// 1    1    2    2
/// 0    5    0    5    0    5
/// 9 ...#########################...
/// 10 ..####B######################..
/// 11 .###S#############.###########.
/// In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.
///
/// Consult the report from the sensors you just deployed. In the row where y=2000000,
/// how many positions cannot contain a beacon?
fn cannot_contain_beacon(sensors: Vec<Sensor>, row: i32) -> usize {
    let mut map_range: GridRange = GridRange {
        x_range: (0, 0),
        y_range: (0, 0),
    };
    sensors
        .iter()
        .for_each(|s| map_range.expand_range(s.get_largest_range()));
    let beacon_pos = sensors
        .iter()
        .map(|s| s.closest_beacon.pos)
        .collect::<Vec<GridPos>>();

    let mut point_reached: usize = 0;
    let mut point_not_reached: usize = 0;
    for x in (map_range.x_range.0..=map_range.x_range.1) {
        let point = GridPos { x: x, y: row };
        match sensors.iter().find(|s| s.covers_point(&point)) {
            Some(_) => {
                if !beacon_pos.contains(&point) {
                    point_reached += 1;
                }
            }
            None => point_not_reached += 1,
        }
    }

    point_reached
}

fn read_sensor_data(data: String) -> Vec<Sensor> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut sensors = Vec::new();

    for line in data.lines() {
        let caps = re
            .captures(line)
            .expect(&format!("Error with line: {line}"));
        let sensor_x = caps
            .get(1)
            .expect("first capture not found")
            .as_str()
            .parse::<i32>()
            .expect("Cannot parse sensor x");
        let sensor_y = caps
            .get(2)
            .expect("second capture not found")
            .as_str()
            .parse::<i32>()
            .expect("Cannot parse sensor y");
        let beacon_x = caps
            .get(3)
            .expect("third capture not found")
            .as_str()
            .parse::<i32>()
            .expect("Cannot parse beacon x");
        let beacon_y = caps
            .get(4)
            .expect("fourth capture not found")
            .as_str()
            .parse::<i32>()
            .expect("Cannot parse beacon y");

        sensors.push(Sensor {
            pos: GridPos {
                x: sensor_x,
                y: sensor_y,
            },
            closest_beacon: Beacon {
                pos: GridPos {
                    x: beacon_x,
                    y: beacon_y,
                },
            },
        });
    }

    sensors
}

#[derive(Debug)]
struct Sensor {
    pos: GridPos,
    closest_beacon: Beacon,
}

#[derive(Debug)]
struct GridRange {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl GridRange {
    fn expand_range(&mut self, range: GridRange) {
        // lowest x
        if self.x_range.0 > range.x_range.0 {
            self.x_range.0 = range.x_range.0;
        }
        // highest x
        if self.x_range.1 < range.x_range.1 {
            self.x_range.1 = range.x_range.1;
        }
        // lowest y
        if self.y_range.0 > range.y_range.0 {
            self.y_range.0 = range.y_range.0;
        }
        // highest y
        if self.y_range.1 < range.y_range.1 {
            self.y_range.1 = range.y_range.1;
        }
    }
}

impl Sensor {
    fn get_largest_range(&self) -> GridRange {
        // find taxicab distance
        let distance = (self.pos.x - self.closest_beacon.pos.x).abs()
            + (self.pos.y - self.closest_beacon.pos.y).abs();
        GridRange {
            x_range: (self.pos.x - distance, self.pos.x + distance),
            y_range: (self.pos.y - distance, self.pos.y + distance),
        }
    }

    fn covers_point(&self, point: &GridPos) -> bool {
        let sensor_distance = (self.pos.x - self.closest_beacon.pos.x).abs()
            + (self.pos.y - self.closest_beacon.pos.y).abs();

        let distance_to_point = (self.pos.x - point.x).abs() + (self.pos.y - point.y).abs();

        sensor_distance >= distance_to_point
    }

    fn get_edge_points(&self) -> Vec<GridPos> {
        let sensor_distance = (self.pos.x - self.closest_beacon.pos.x).abs()
            + (self.pos.y - self.closest_beacon.pos.y).abs();
        let mut oob_points = Vec::with_capacity((sensor_distance * 8) as usize);

        for x in (0..sensor_distance + 1) {
            let y = sensor_distance + 1 - x;
            oob_points.push(GridPos { x: (x), y: (y) });
            oob_points.push(GridPos { x: (x), y: (-y) });
            oob_points.push(GridPos { x: (-x), y: (y) });
            oob_points.push(GridPos { x: (-x), y: (-y) });
        }

        oob_points
    }
}

#[derive(Debug, Copy, Clone)]
struct Beacon {
    pos: GridPos,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct GridPos {
    x: i32,
    y: i32,
}

impl GridPos {
    fn get_tuning_freqency(&self) -> i32 {
        self.x * 4000000 + self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_non_beacon_locations() {
        let data = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let sensors = read_sensor_data(data.into());
        assert_eq!(26, cannot_contain_beacon(sensors, 10));
    }

    #[test]
    fn test_find_uncovered_beacons() {
        let data = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let sensors = read_sensor_data(data.into());
        let search_space = GridRange {
            x_range: (0, 20),
            y_range: (0, 20),
        };
        let point = find_no_beacon_zone(sensors, search_space);
        assert_eq!(56000011, point.get_tuning_freqency());
    }
}
