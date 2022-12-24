use crate::utils::read_data;
use std::fmt;

const FILE: &str = "day21.txt";
const DAY: &str = "{{ DAY 21 }}";

/// --- Day 22: Monkey Map ---
pub fn part1() {
    let data = read_data(FILE);
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
pub fn part2() {
    let data = read_data(FILE);
}

fn read_map(data: String) -> DataMap {
    let last_col = data
        .lines()
        .filter_map(|line| line.bytes().rposition(|v| v == b'.' || v == b'#'))
        .min()
        .unwrap();
    let map_height = data
        .lines()
        .filter(|line| line.contains('.') || line.contains('#'))
        .count();
    let mut path = String::new();
    let mut map = vec![vec![Cell::Void; last_col]; map_height];

    for (row, line) in data.lines().enumerate() {
        if line.contains('.') || line.contains('#') {
            let line_bytes = line.as_bytes();
            for i in 0..last_col {
                match line_bytes.get(i) {
                    Some(b'.') => map[row][i] = Cell::Open,
                    Some(b'#') => map[row][i] = Cell::Wall,
                    _ => {}
                }
            }

            // still reading map
        } else if line.contains("R") || line.contains("L") {
            // reading path instructions
            path.push_str(line);
        }
    }

    // the upper left most open space
    let first_pos = map[0].iter().position(|c| c == &Cell::Open).unwrap();

    DataMap {
        map,
        path,
        pos: (first_pos, 0),
        direction: Direction::Right,
    }
}

fn execute(map: DataMap, path: String) -> String {
    // parse path
    // for instruction in path
    // call on map
    // return current postion
    todo!()
}

struct DataMap {
    map: Vec<Vec<Cell>>,
    path: String,
    pos: (usize, usize), // (x, y) position of the player
    direction: Direction,
}
impl DataMap {
    fn print_map(&self) -> String {
        let mut s = String::new();
        self.map.iter().for_each(|row| {
            row.iter().for_each(|c| match c {
                Cell::Open => s.push('.'),
                Cell::Wall => s.push('#'),
                Cell::Void => s.push(' '),
            });
            s.push('\n');
        });
        s
    }

    fn get_status(&self) -> String {
        format!(
            "Current pos ({}, {}) facing {}",
            self.pos.0 + 1,
            self.pos.1 + 1,
            self.direction
        )
    }

    fn move_forward(&mut self, num: usize) {
        for _ in 0..num {
            self.take_step()
        }
    }

    fn take_step(&mut self) {
        match self.direction {
            Direction::Down => self.step_down(),
            Direction::Left => self.step_left(),
            Direction::Up => self.step_up(),
            Direction::Right => self.step_right(),
        }
    }

    fn step_right(&mut self) {
        todo!()
    }
    fn step_left(&mut self) {
        todo!()
    }
    fn step_up(&mut self) {
        todo!()
    }
    fn step_down(&mut self) {
        todo!()
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Open,
    Void,
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Down => write!(f, "Direction::Down"),
            Direction::Left => write!(f, "Direction::Left"),
            Direction::Up => write!(f, "Direction::Up"),
            Direction::Right => write!(f, "Direction::Right"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_map() {
        let data = "        ...#    
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let map = read_map(data.into());
        println!("str:\n{}", map.print_map());
        assert_eq!(map.map.len(), 12);
        assert_eq!(map.path, String::from("10R5L5R10L4R5L5"));
        assert_eq!(map.map[0].len(), 16);
        assert_eq!(map.pos, (8, 0))
    }
}
