use crate::read_from_data_dir;
use std::collections::HashSet;

/// # --- Day 15: Chiton ---
/// You've almost reached the exit of the cave, but the walls are getting closer together. Your submarine can barely still fit, though; the main problem is that the walls of the cave are covered in chitons, and it would be best not to bump any of them.
///
/// The cavern is large, but has a very low ceiling, restricting your motion to two dimensions. The shape of the cavern resembles a square; a quick scan of chiton density produces a map of risk level throughout the cave (your puzzle input). For example:
/// ```text
/// 1163751742
/// 1381373672
/// 2136511328
/// 3694931569
/// 7463417111
/// 1319128137
/// 1359912421
/// 3125421639
/// 1293138521
/// 2311944581
/// ```
/// You start in the top left position, your destination is the bottom right position, and you cannot move diagonally. The number at each position is its risk level; to determine the total risk of an entire path, add up the risk levels of each position you enter (that is, don't count the risk level of your starting position unless you enter it; leaving it adds no risk to your total).
///
/// Your goal is to find a path with the lowest total risk. In this example, a path with the lowest total risk is highlighted here:
/// ```text
/// 1163751742
/// 1381373672
/// 2136511328
/// 3694931569
/// 7463417111
/// 1319128137
/// 1359912421
/// 3125421639
/// 1293138521
/// 2311944581
/// ```
/// The total risk of this path is 40 (the starting position is never entered, so its risk is not counted).
///
/// What is the lowest total risk of any path from the top left to the bottom right?
pub fn part1() {
    let map = string_to_map(read_from_data_dir("day15.txt").unwrap());
    let min = find_min_path(map);
    println!("Day15:1. Min path is {}", min);
}

fn string_to_map(data: String) -> Vec<Vec<u8>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("bad parsing") as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

/// Helpful for not backtracking and remembering where the algorithm has been to
/// Its also a built in cache
#[derive(std::hash::Hash, Copy, Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
    travel_cost: u64,
}

impl Node {
    fn up(&self, map: &Vec<Vec<u8>>) -> Option<Self> {
        if self.y > 0 {
            let new_y = self.y - 1;
            return Some(Node {
                x: self.x,
                y: new_y,
                travel_cost: self.travel_cost + map[new_y][self.x] as u64,
            });
        }
        None
    }

    fn down(&self, map: &Vec<Vec<u8>>) -> Option<Self> {
        if self.y < map.len() - 1 {
            let new_y = self.y + 1;
            return Some(Node {
                x: self.x,
                y: new_y,
                travel_cost: self.travel_cost + map[new_y][self.x] as u64,
            });
        }
        None
    }

    fn left(&self, map: &Vec<Vec<u8>>) -> Option<Self> {
        if self.x > 0 {
            let new_x = self.x - 1;
            return Some(Node {
                x: new_x,
                y: self.y,
                travel_cost: self.travel_cost + map[self.y][new_x] as u64,
            });
        }
        None
    }

    fn right(&self, map: &Vec<Vec<u8>>) -> Option<Self> {
        if self.x < map[0].len() - 1 {
            let new_x = self.x + 1;
            return Some(Node {
                x: new_x,
                y: self.y,
                travel_cost: self.travel_cost + map[self.y][new_x] as u64,
            });
        }
        None
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}

/// This is a dijkstra's algorithm
fn find_min_path(map: Vec<Vec<u8>>) -> u64 {
    // starting out at the top left
    let mut current_position: (usize, usize) = (0, 0);
    // ending once the current position gets to the bottom right
    let ending_postition = (map[0].len() - 1, map.len() - 1);
    // nodes we have been to but haven't left. These will be compared to each other to find which is the next best step to take
    let mut paused_positions: HashSet<(usize, usize)> = HashSet::new();
    paused_positions.insert(current_position);
    // nodes that have been visited and should be considered again
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    // initializing the min spanning tree
    let mut min_span_grid: Vec<Vec<Node>> = vec![
        vec![
            Node {
                x: 0,
                y: 0,
                travel_cost: std::u64::MAX
            };
            map[0].len()
        ];
        map.len()
    ];
    // fixing the x and y from the vector initialization
    for (y, row) in map.iter().enumerate() {
        for (x, _val) in row.iter().enumerate() {
            min_span_grid[y][x] = Node {
                x: x,
                y: y,
                travel_cost: std::u64::MAX,
            };
        }
    }
    // fixing the starting position from the vector initialization
    min_span_grid[current_position.1][current_position.0].travel_cost = 0;

    while current_position != ending_postition {
        let mut min_next_step = std::u64::MAX;
        let mut next_best_position: (usize, usize) = current_position;
        // making a clone every round so i can alter paused_positions
        let current_paused_positions = paused_positions.clone();
        for (x, y) in &current_paused_positions {
            let node: &Node = &min_span_grid[*y][*x];
            let possible_locations_to_go: Vec<Option<Node>> = vec![
                node.up(&map),
                node.down(&map),
                node.left(&map),
                node.right(&map),
            ];
            // filter out None, or already visited positions
            let possible_locations_to_go: Vec<Node> = possible_locations_to_go
                .iter()
                .filter(|node| node.is_some())
                .map(|node| node.unwrap())
                .filter(|node| !visited_positions.contains(&(node.x, node.y)))
                .collect();

            // finding the cheapest location to move to
            let cost = possible_locations_to_go
                .iter()
                .map(|node| node.travel_cost)
                .min()
                .unwrap();
            // finding the cheapest location's position
            let cheapest_step = possible_locations_to_go
                .iter()
                .filter(|node| node.travel_cost == cost)
                .map(|node| (node.x, node.y))
                .collect::<Vec<(usize, usize)>>()[0];
            // putting these new locations into consideration for further exploration
            possible_locations_to_go.iter().for_each(|node| {
                let _ = paused_positions.insert((node.x, node.y));

                // update the min span grid
                if min_span_grid[node.y][node.x].travel_cost > node.travel_cost {
                    min_span_grid[node.y][node.x] = Node {
                        x: node.x,
                        y: node.y,
                        travel_cost: node.travel_cost,
                    };
                }
            });

            let _removed = paused_positions.remove(&(*x, *y));
            let _ = visited_positions.insert((*x, *y));

            // update the loops next step values
            if cost < min_next_step {
                min_next_step = cost;
                next_best_position = cheapest_step
            };
        }
        current_position = next_best_position;
    }

    // return the final node in the bottom right corner
    min_span_grid[ending_postition.1][ending_postition.0].travel_cost
}

pub fn part2() {}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_min_path_1() {
        let map = vec![vec![1, 1, 6], vec![1, 3, 8], vec![2, 1, 3]];
        assert_eq!(find_min_path(map), 7);
    }

    #[test]
    fn test_find_min_path_2() {
        let map = vec![vec![7, 9, 2], vec![4, 6, 5], vec![1, 9, 8]];
        assert_eq!(find_min_path(map), 22);
    }

    #[test]
    fn test_find_min_path_3() {
        let map = string_to_map(String::from("1163751742\n1381373672\n2136511328\n3694931569\n7463417111\n1319128137\n1359912421\n3125421639\n1293138521\n2311944581"));
        assert_eq!(find_min_path(map), 40);
    }
}
