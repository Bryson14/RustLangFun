use crate::read_from_data_dir;
use regex::Regex;
use std::fmt;

/// --- Day 13: Transparent Origami ---
/// You reach another volcanically active part of the cave. It would be nice if you could do some kind of thermal imaging so you could tell ahead of time which caves are too hot to safely enter.
///
/// Fortunately, the submarine seems to be equipped with a thermal camera! When you activate it, you are greeted with:
///
/// Congratulations on your purchase! To activate this infrared thermal imaging
/// camera system, please enter the code found on page 1 of the manual.
/// Apparently, the Elves have never used this feature. To your surprise, you manage to find the manual; as you go to open it, page 1 falls out. It's a large sheet of transparent paper! The transparent paper is marked with random dots and includes instructions on how to fold it up (your puzzle input). For example:
///
/// 6,10
/// 0,14
/// 9,10
/// 0,3
/// 10,4
/// 4,11
/// 6,0
/// 6,12
/// 4,1
/// 0,13
/// 10,12
/// 3,4
/// 3,0
/// 8,4
/// 1,10
/// 2,14
/// 8,10
/// 9,0
///
/// fold along y=7
/// fold along x=5
/// The first section is a list of dots on the transparent paper. 0,0 represents the top-left coordinate. The first value, x, increases to the right. The second value, y, increases downward. So, the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example form the following pattern, where # is a dot on the paper and . is an empty, unmarked position:
///
/// ...#..#..#.
/// ....#......
/// ...........
/// #..........
/// ...#....#.#
/// ...........
/// ...........
/// ...........
/// ...........
/// ...........
/// .#....#.##.
/// ....#......
/// ......#...#
/// #..........
/// #.#........
/// Then, there is a list of fold instructions. Each instruction indicates a line on the transparent paper and wants you to fold the paper up (for horizontal y=... lines) or left (for vertical x=... lines). In this example, the first fold instruction is fold along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):
///
/// ...#..#..#.
/// ....#......
/// ...........
/// #..........
/// ...#....#.#
/// ...........
/// ...........
/// -----------
/// ...........
/// ...........
/// .#....#.##.
/// ....#......
/// ......#...#
/// #..........
/// #.#........
/// Because this is a horizontal line, fold the bottom half up. Some of the dots might end up overlapping after the fold is complete, but dots will never appear exactly on a fold line. The result of doing this fold looks like this:
///
/// #.##..#..#.
/// #...#......
/// ......#...#
/// #...#......
/// .#.#..#.###
/// ...........
/// ...........
/// Now, only 17 dots are visible.
///
/// Notice, for example, the two dots in the bottom left corner before the transparent paper is folded; after the fold is complete, those dots appear in the top left corner (at 0,0 and 0,1). Because the paper is transparent, the dot just below them in the result (at 0,3) remains visible, as it can be seen through the transparent paper.
///
/// Also notice that some dots can end up overlapping; in this case, the dots merge together and become a single dot.
///
/// The second fold instruction is fold along x=5, which indicates this line:
///
/// #.##.|#..#.
/// #...#|.....
/// .....|#...#
/// #...#|.....
/// .#.#.|#.###
/// .....|.....
/// .....|.....
/// Because this is a vertical line, fold left:
///
/// #####
/// #...#
/// #...#
/// #...#
/// #####
/// .....
/// .....
/// The instructions made a square!
///
/// The transparent paper is pretty big, so for now, focus on just completing the first fold. After the first fold in the example above, 17 dots are visible - dots that end up overlapping after the fold is completed count as a single dot.
///
/// How many dots are visible after completing just the first fold instruction on your transparent paper?
pub fn part1() {
    let (positions_of_dots, _list_of_cuts) =
        read_origami_paper(read_from_data_dir("day13.txt").unwrap());
    let _origami_paper = OrgPaper::from_positions(positions_of_dots);
    // println!("paper: {}", origami_paper);
}

fn read_origami_paper(s: String) -> (Vec<(i32, i32)>, Vec<Cut>) {
    let mut positions: Vec<(i32, i32)> = Vec::new();
    let mut cuts: Vec<Cut> = Vec::new();
    let position_re = Regex::new(r"^[0-9]+,[0-9]+$").unwrap();
    let fold_re = Regex::new(r"fold along").unwrap();

    for line in s.lines() {
        if position_re.is_match(line) {
            let line_x_y: Vec<i32> = line
                .split(',')
                .map(|s| s.parse::<i32>().expect("error parsing number"))
                .collect();
            positions.push((line_x_y[0], line_x_y[1]));
        } else if fold_re.is_match(line) {
            let contains_x = line.contains('x');
            let equal_sign = line.chars().position(|c| c == '=').unwrap();
            let num = &line[equal_sign + 1..line.len()];
            let num = num.parse::<i32>().unwrap();
            cuts.push(Cut {
                idx: num,
                x: contains_x,
            });
        } else {
            println!("Empty Line");
        }
    }

    (positions, cuts)
}

#[derive(Debug, PartialEq)]
struct Cut {
    idx: i32,
    x: bool,
}

#[derive(Debug, PartialEq)]
struct OrgPaper {
    grid: Vec<Vec<bool>>,
}

impl OrgPaper {
    fn from_positions(positions: Vec<(i32, i32)>) -> OrgPaper {
        let max_x = positions.iter().map(|pos| pos.0).max().unwrap();
        let max_y = positions.iter().map(|pos| pos.1).max().unwrap();

        let mut grid: Vec<Vec<bool>> =
            vec![vec![false; (max_x + 1) as usize]; (max_y + 1) as usize];
        positions.iter().for_each(|pos| {
            grid[pos.1 as usize][pos.0 as usize] = true;
        });

        OrgPaper { grid }
    }
}

impl fmt::Display for OrgPaper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|&item| {
                if item {
                    s.push_str("# ")
                } else {
                    s.push_str(". ")
                }
            });
            s.push('\n');
        });
        write!(f, "{}", s)
    }
}

pub fn part2() {}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_origami_paper() {
        let s = String::from("1,1\n2,2\nfold along x=5\n");
        assert_eq!(
            read_origami_paper(s),
            (vec![(1, 1), (2, 2)], vec![Cut { idx: 5, x: true }])
        );
    }

    #[test]
    fn test_org_paper_from_positions() {
        let pos = vec![(1, 1), (0, 0)];
        let grid = vec![vec![true, false], vec![false, true]];
        assert_eq!(OrgPaper::from_positions(pos), OrgPaper { grid: grid });
    }
}
