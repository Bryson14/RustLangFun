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
    println!("Day15:1. Min path is {} 508 actually", min);
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
#[derive(Copy, Clone, Debug)]
struct Node {
    x: usize,
    y: usize,
    travel_cost: u64,
}

impl Node {
    fn up(&self, map: &[Vec<u8>]) -> Option<Self> {
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

    fn down(&self, map: &[Vec<u8>]) -> Option<Self> {
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

    fn left(&self, map: &[Vec<u8>]) -> Option<Self> {
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

    fn right(&self, map: &[Vec<u8>]) -> Option<Self> {
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
                x,
                y,
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
                .flatten()
                .filter(|node| !visited_positions.contains(&(node.x, node.y)))
                .copied()
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

/// # --- Part Two ---
/// Now that you know how to find low-risk paths in the cave, you can try to find your way out.
///
/// The entire cave is actually five times larger in both dimensions than you thought; the area you originally scanned is just one tile in a 5x5 tile area that forms the full map. Your original map tile repeats to the right and downward; each time the tile repeats to the right or downward, all of its risk levels are 1 higher than the tile immediately up or left of it. However, risk levels above 9 wrap back around to 1. So, if your original map had some position with a risk level of 8, then that same position on each of the 25 total tiles would be as follows:
///
/// 8 9 1 2 3
/// 9 1 2 3 4
/// 1 2 3 4 5
/// 2 3 4 5 6
/// 3 4 5 6 7
/// Each single digit above corresponds to the example position with a value of 8 on the top-left tile. Because the full map is actually five times larger in both dimensions, that position appears a total of 25 times, once in each duplicated tile, with the values shown above.
///
/// Here is the full five-times-as-large version of the first example above, with the original map in the top left corner highlighted:
///
/// 11637517422274862853338597396444961841755517295286
/// 13813736722492484783351359589446246169155735727126
/// 21365113283247622439435873354154698446526571955763
/// 36949315694715142671582625378269373648937148475914
/// 74634171118574528222968563933317967414442817852555
/// 13191281372421239248353234135946434524615754563572
/// 13599124212461123532357223464346833457545794456865
/// 31254216394236532741534764385264587549637569865174
/// 12931385212314249632342535174345364628545647573965
/// 23119445813422155692453326671356443778246755488935
/// 22748628533385973964449618417555172952866628316397
/// 24924847833513595894462461691557357271266846838237
/// 32476224394358733541546984465265719557637682166874
/// 47151426715826253782693736489371484759148259586125
/// 85745282229685639333179674144428178525553928963666
/// 24212392483532341359464345246157545635726865674683
/// 24611235323572234643468334575457944568656815567976
/// 42365327415347643852645875496375698651748671976285
/// 23142496323425351743453646285456475739656758684176
/// 34221556924533266713564437782467554889357866599146
/// 33859739644496184175551729528666283163977739427418
/// 35135958944624616915573572712668468382377957949348
/// 43587335415469844652657195576376821668748793277985
/// 58262537826937364893714847591482595861259361697236
/// 96856393331796741444281785255539289636664139174777
/// 35323413594643452461575456357268656746837976785794
/// 35722346434683345754579445686568155679767926678187
/// 53476438526458754963756986517486719762859782187396
/// 34253517434536462854564757396567586841767869795287
/// 45332667135644377824675548893578665991468977611257
/// 44961841755517295286662831639777394274188841538529
/// 46246169155735727126684683823779579493488168151459
/// 54698446526571955763768216687487932779859814388196
/// 69373648937148475914825958612593616972361472718347
/// 17967414442817852555392896366641391747775241285888
/// 46434524615754563572686567468379767857948187896815
/// 46833457545794456865681556797679266781878137789298
/// 64587549637569865174867197628597821873961893298417
/// 45364628545647573965675868417678697952878971816398
/// 56443778246755488935786659914689776112579188722368
/// 55172952866628316397773942741888415385299952649631
/// 57357271266846838237795794934881681514599279262561
/// 65719557637682166874879327798598143881961925499217
/// 71484759148259586125936169723614727183472583829458
/// 28178525553928963666413917477752412858886352396999
/// 57545635726865674683797678579481878968159298917926
/// 57944568656815567976792667818781377892989248891319
/// 75698651748671976285978218739618932984172914319528
/// 56475739656758684176786979528789718163989182927419
/// 67554889357866599146897761125791887223681299833479
/// Equipped with the full map, you can now find a path from the top left corner to the bottom right corner with the lowest total risk:
///
/// 11637517422274862853338597396444961841755517295286
/// 13813736722492484783351359589446246169155735727126
/// 21365113283247622439435873354154698446526571955763
/// 36949315694715142671582625378269373648937148475914
/// 74634171118574528222968563933317967414442817852555
/// 13191281372421239248353234135946434524615754563572
/// 13599124212461123532357223464346833457545794456865
/// 31254216394236532741534764385264587549637569865174
/// 12931385212314249632342535174345364628545647573965
/// 23119445813422155692453326671356443778246755488935
/// 22748628533385973964449618417555172952866628316397
/// 24924847833513595894462461691557357271266846838237
/// 32476224394358733541546984465265719557637682166874
/// 47151426715826253782693736489371484759148259586125
/// 85745282229685639333179674144428178525553928963666
/// 24212392483532341359464345246157545635726865674683
/// 24611235323572234643468334575457944568656815567976
/// 42365327415347643852645875496375698651748671976285
/// 23142496323425351743453646285456475739656758684176
/// 34221556924533266713564437782467554889357866599146
/// 33859739644496184175551729528666283163977739427418
/// 35135958944624616915573572712668468382377957949348
/// 43587335415469844652657195576376821668748793277985
/// 58262537826937364893714847591482595861259361697236
/// 96856393331796741444281785255539289636664139174777
/// 35323413594643452461575456357268656746837976785794
/// 35722346434683345754579445686568155679767926678187
/// 53476438526458754963756986517486719762859782187396
/// 34253517434536462854564757396567586841767869795287
/// 45332667135644377824675548893578665991468977611257
/// 44961841755517295286662831639777394274188841538529
/// 46246169155735727126684683823779579493488168151459
/// 54698446526571955763768216687487932779859814388196
/// 69373648937148475914825958612593616972361472718347
/// 17967414442817852555392896366641391747775241285888
/// 46434524615754563572686567468379767857948187896815
/// 46833457545794456865681556797679266781878137789298
/// 64587549637569865174867197628597821873961893298417
/// 45364628545647573965675868417678697952878971816398
/// 56443778246755488935786659914689776112579188722368
/// 55172952866628316397773942741888415385299952649631
/// 57357271266846838237795794934881681514599279262561
/// 65719557637682166874879327798598143881961925499217
/// 71484759148259586125936169723614727183472583829458
/// 28178525553928963666413917477752412858886352396999
/// 57545635726865674683797678579481878968159298917926
/// 57944568656815567976792667818781377892989248891319
/// 75698651748671976285978218739618932984172914319528
/// 56475739656758684176786979528789718163989182927419
/// 67554889357866599146897761125791887223681299833479
/// The total risk of this path is 315 (the starting position is still never entered, so its risk is not counted).
///
/// Using the full map, what is the lowest total risk of any path from the top left to the bottom right?
pub fn part2() {
    let map = string_to_map(read_from_data_dir("day15.txt").unwrap());
    let min = find_min_path(duplicate_map(map, 5));
    println!(
        "Day15:2. Min path is {} 2875 is too high, 2865 too low, 2873,2870",
        min
    );
}

/// the original map is retiled and multiplied to be a larger map.assert_eq!
/// Everytime the map is retiled down or to the right, the numbers all increase by one (9s go back to 1)
fn duplicate_map(mut map: Vec<Vec<u8>>, size_increase: usize) -> Vec<Vec<u8>> {
    // retiling horizontally first
    let original_copy = map.clone();
    let og_height = map.len();
    for n in 1..size_increase {
        for (idx, row) in original_copy.iter().enumerate() {
            for col in row.iter() {
                map[idx].push(increase_threat_level(*col, n));
            }
        }
    }
    //retiling now vertically
    for n in 1..size_increase {
        for (idx, _row) in original_copy.iter().enumerate() {
            map.push(
                map[og_height * (n - 1) + idx]
                    .iter()
                    .map(|num| increase_threat_level(*num, 1))
                    .collect::<Vec<u8>>(),
            );
        }
    }

    map
}

/// used to deuplicate the map. A number increases to 9. When a 9 is passed in, it turns into a 1 again
/// n is the number of times to increase it.
fn increase_threat_level(num: u8, n: usize) -> u8 {
    if n == 0 {
        return num;
    }
    let new = match num {
        n @ 1..=8 => n + 1,
        9 => 1,
        _ => unreachable!(),
    };
    increase_threat_level(new, n - 1)
}
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

    #[test]
    fn test_find_min_path_4() {
        let map = string_to_map(String::from("1111111111\n1181373671\n2136511321\n3694931561\n7463417111\n1319128131\n1359912421\n3125421631\n1293138521\n2311944581"));
        assert_eq!(find_min_path(map), 18);
    }

    #[test]
    fn test_duplicate_map() {
        let map = string_to_map(read_from_data_dir("day15_test_og_map.txt").unwrap());
        let larger_map = string_to_map(read_from_data_dir("day15_test_larger_map.txt").unwrap());
        assert_eq!(duplicate_map(map, 5), larger_map);
    }

    #[test]
    fn test_find_min_path_5() {
        let larger_map = string_to_map(read_from_data_dir("day15_test_larger_map.txt").unwrap());
        assert_eq!(find_min_path(larger_map), 315);
    }

    #[test]
    fn test_inrease_threat_level() {
        assert_eq!(increase_threat_level(5, 1), 6);
        assert_eq!(increase_threat_level(5, 2), 7);
        assert_eq!(increase_threat_level(5, 3), 8);
        assert_eq!(increase_threat_level(5, 4), 9);
        assert_eq!(increase_threat_level(5, 5), 1);
        assert_eq!(increase_threat_level(5, 0), 5);
    }
}
