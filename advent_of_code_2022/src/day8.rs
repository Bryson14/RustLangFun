use crate::utils::read_data;
use std::collections::HashSet;

const FILE: &str = "day8.txt";
const DAY: &str = "{{ DAY 8 }}";

/// --- Day 8: Treetop Tree House ---
/// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
/// Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
/// Consider your map; how many trees are visible from outside the grid?
pub fn part1() {
    let data = read_data(FILE);
    let mut trees = read_grid(data);
    trees = find_all_visible_trees(trees);
    println!(
        "{DAY}-1 number of trees visible from edges is {}",
        trees.visible.len()
    );
}

/// Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house:
/// they would like to be able to see a lot of trees.
/// A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
///
/// However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
///
/// 30373
/// 25512
/// 65332
/// 33549
/// 35390
/// Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
/// Looking left, its view is not blocked; it can see 2 trees.
/// Looking down, its view is also not blocked; it can see 1 tree.
/// Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
/// This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
///
/// Consider each tree on your map. What is the highest scenic score possible for any tree?
pub fn part2() {
    let data = read_data(FILE);
    let trees = read_grid(data);
    let max_scenic = find_best_scenic_score(&trees);
    println!("{DAY}-2 The best scenic rating view is {max_scenic}.");
}

fn find_best_scenic_score(trees: &Trees) -> usize {
    let mut max_scenic = 0;
    for row in 0..trees.grid.len() {
        for col in 0..trees.grid[0].len() {
            let rating = get_scenic_score(trees, (row, col));
            if rating > max_scenic {
                max_scenic = rating;
            }
        }
    }
    max_scenic
}

fn get_scenic_score(trees: &Trees, pos: (usize, usize)) -> usize {
    let funcs = [see_down, see_up, see_right, see_left];
    funcs.iter().map(|f| f(trees, pos)).product()
}

fn see_up(trees: &Trees, pos: (usize, usize)) -> usize {
    let mut score = 0;
    let val = trees.grid[pos.0][pos.1];
    for row in (0..pos.0).rev() {
        score += 1;
        if val <= trees.grid[row][pos.1] {
            break;
        }
    }
    score
}

fn see_down(trees: &Trees, pos: (usize, usize)) -> usize {
    let map_height = trees.grid.len();
    let mut score = 0;
    let val = trees.grid[pos.0][pos.1];
    for row in pos.0 + 1..map_height {
        score += 1;
        if val <= trees.grid[row][pos.1] {
            break;
        }
    }
    score
}

fn see_right(trees: &Trees, pos: (usize, usize)) -> usize {
    let map_width = trees.grid[0].len();
    let mut score = 0;
    let val = trees.grid[pos.0][pos.1];
    for col in pos.1 + 1..map_width {
        score += 1;
        if val <= trees.grid[pos.0][col] {
            break;
        }
    }
    score
}

fn see_left(trees: &Trees, pos: (usize, usize)) -> usize {
    let mut score = 0;
    let val = trees.grid[pos.0][pos.1];
    for col in (0..pos.1).rev() {
        score += 1;
        if val <= trees.grid[pos.0][col] {
            break;
        }
    }
    score
}

#[derive(Debug)]
struct Trees {
    grid: Vec<Vec<u8>>,
    visible: HashSet<(usize, usize)>,
}

enum FromView {
    Right,
    Left,
    Top,
    Bottom,
}

fn find_all_visible_trees(mut trees: Trees) -> Trees {
    let funcs = [visible_right, visible_left, visible_top, visible_bottom];
    for f in funcs {
        trees = f(trees);
    }
    trees
}

fn visible_top(mut trees: Trees) -> Trees {
    let mut set = trees.visible.clone();
    for col in 0..trees.grid[0].len() {
        let mut tallest = 0;
        for row in 0..trees.grid.len() {
            let h = trees.grid[row][col];
            if h > tallest || row == 0 {
                tallest = h;
                set.insert((row, col));
            }
        }
    }
    trees.visible = set;
    trees
}

fn visible_bottom(mut trees: Trees) -> Trees {
    let mut set = trees.visible.clone();
    let grid_height = trees.grid.len();
    for col in 0..trees.grid[0].len() {
        let mut tallest = 0;
        for row in (0..trees.grid.len()).rev() {
            let h = trees.grid[row][col];
            if h > tallest || row == grid_height - 1 {
                tallest = h;
                set.insert((row, col));
            }
        }
    }
    trees.visible = set;
    trees
}

fn visible_right(mut trees: Trees) -> Trees {
    let mut set = trees.visible.clone();
    let grid_width = trees.grid[0].len();
    for row in 0..trees.grid.len() {
        let mut tallest = 0;
        for col in (0..grid_width).rev() {
            let h = trees.grid[row][col];
            if h > tallest || col == grid_width - 1 {
                tallest = h;
                set.insert((row, col));
            }
        }
    }
    trees.visible = set;
    trees
}

fn visible_left(mut trees: Trees) -> Trees {
    let mut set = trees.visible.clone();
    let grid_width = trees.grid[0].len();
    for row in 0..trees.grid.len() {
        let mut tallest = 0;
        for col in 0..grid_width {
            let h = trees.grid[row][col];
            if h > tallest || col == 0 {
                tallest = h;
                set.insert((row, col));
            }
        }
    }
    trees.visible = set;
    trees
}

fn read_grid(data: String) -> Trees {
    let grid = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c as u8)
                .filter(|b| b >= &48 && b <= &57)
                .map(|b| b - ('0' as u8))
                .collect()
        })
        .collect();
    Trees {
        grid,
        visible: HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_edge() {
        let data = "
        30373
        25512
        65332
        33549
        35390";
        let trees = read_grid(data.into());
        let trees = find_all_visible_trees(trees);
        assert!(trees.visible.len() == 21);
    }

    #[test]
    fn test_visible_top() {
        // 2 3 1
        // 1 1 6
        // 9 5 7
        let grid = vec![vec![2, 3, 1], vec![1, 1, 6], vec![9, 5, 7]];
        let trees = Trees {
            grid,
            visible: HashSet::new(),
        };
        let trees = visible_top(trees);
        let mut ans = HashSet::new();
        ans.extend([(1, 2), (0, 0), (0, 1), (0, 2), (2, 1), (2, 2), (2, 0)]);
        println!("{:?}", trees.visible);
        assert!(trees.visible == ans);
    }

    #[test]
    fn test_visible_bottom() {
        // 2 3 1
        // 1 1 6
        // 9 5 7
        let grid = vec![vec![2, 3, 1], vec![1, 1, 6], vec![9, 5, 7]];
        let trees = Trees {
            grid,
            visible: HashSet::new(),
        };
        let trees = visible_bottom(trees);
        let mut ans = HashSet::new();
        ans.extend([(2, 1), (2, 2), (2, 0)]);
        println!("{:?}", trees.visible);
        assert!(trees.visible == ans);
    }

    #[test]
    fn test_find_scenic() {
        let data = "
        30373
        25512
        65332
        33549
        35390";
        let trees = read_grid(data.into());
        assert_eq!(see_up(&trees, (3, 2)), 2);
        assert_eq!(see_down(&trees, (3, 2)), 1);
        assert_eq!(see_right(&trees, (3, 2)), 2);
        assert_eq!(see_left(&trees, (3, 2)), 2);
        let ans = find_best_scenic_score(&trees);
        println!("ans: {ans}");
        assert!(ans == 8);
    }
}
