use std::collections::{HashMap, HashSet};
use super::input::{EngineSymbol, EngineSymbolType, Map, read_to_map, get_engine_symbol_map};
/// # --- Part Two ---
/// The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.
///
/// You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.
///
/// Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.
///
/// The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.
///
/// This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
///
/// ```
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// ```
/// In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.
///
/// What is the sum of all of the gear ratios in your engine schematic?
pub fn solve(input: &str) {
    let map = read_to_map(input);
    let map_lookup = get_engine_symbol_map(&map);

    let sum = sum_gear_ratios(&map, &map_lookup);

    println!("The sum of all of the gear ratios in your engine schematic is {}", sum);

}

fn sum_gear_ratios(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    let mut gear_ratios_sum: u32 = 0;
    let adjacent_neighbors: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    for y in 0..map.height {
        for x in 0..map.width {
            if let Some(symbol) = map_lookup.get(&(x, y)) {
                if let EngineSymbolType::Symbol('*') = symbol.symbol_type {
                    let mut visited_ids: HashSet<usize> = HashSet::new();
                    let mut adjacent_numbers = 0;
                    let mut product = 1;

                    // check all the neighbors
                    for (x2, y2) in adjacent_neighbors.iter() {
                        let x3 = x2 + x as i32;
                        let y3 = y2 + y as i32;

                        if x3 >= 0 && y3 >= 0 && x3 < map.width as i32 && y3 < map.height as i32 {
                            if let Some(adjacent_symbol) = map_lookup.get(&(x3 as usize, y3 as usize)) {
                                if visited_ids.contains(&adjacent_symbol.id) {
                                    continue;
                                }
                                if let EngineSymbolType::Number(num) = adjacent_symbol.symbol_type {
                                    adjacent_numbers += 1;
                                    product *= num;
                                    visited_ids.insert(adjacent_symbol.id); // don't count the same number twice
                                }
                            }
                        }
                    }

                    if adjacent_numbers == 2 {
                        gear_ratios_sum += product;
                    }
                }
            }
        }
    }

    gear_ratios_sum
}

