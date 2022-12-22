use crate::utils::read_data;

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
    let mut map = vec![vec![Cell::Wall; last_col]; map_height];

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

    DataMap { map, path }
}

struct DataMap {
    map: Vec<Vec<Cell>>,
    path: String,
}
impl DataMap {
    fn print_map(&self) -> String {
        let mut s = String::new();
        self.map.iter().for_each(|row| {
            row.iter().for_each(|c| match c {
                Cell::Open => s.push('.'),
                Cell::Wall => s.push('#'),
            });
            s.push('\n');
        });
        s
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Wall,
    Open,
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
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
    }
}
