use super::input::read_to_map;
use crate::day03::input::{get_engine_symbol_map, EngineSymbol, EngineSymbolType, Map};
use std::collections::{HashMap, HashSet};

/// #--- Day 3: Gear Ratios ---
/// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
///
/// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
///
/// "Aaah!"
///
/// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
///
/// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
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
/// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
///
/// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
pub fn solve(input: &str) {
    let map = read_to_map(input);

    // create a hashmap to store the engine symbols, location is going to be a tuple of (x, y) and key
    let engine_symbols = get_engine_symbol_map(&map);

    // iterate through the hashmap and sum all numbers that are adjacent to a symbol
    let sum = sum_adjacent_numbers(&map, &engine_symbols);

    println!("Sum of all non-adjacent numbers: {}", sum);
}

fn sum_adjacent_numbers(map: &Map, map_lookup: &HashMap<(usize, usize), EngineSymbol>) -> u32 {
    // iterate through map and check if the symbol is a number

    let mut visited_ids: HashSet<usize> = HashSet::new();
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
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let symbol = map_lookup.get(&(x, y)).unwrap();
            if let EngineSymbolType::Number(num) = symbol.symbol_type {
                // don't sum the same number twice
                if visited_ids.contains(&symbol.id) {
                    continue;
                }
                // check if the symbol is adjacent to another symbol
                let mut adjacent = false;

                // iterate thorugh the adjacent neighbors and check if they are symbols
                for (x2, y2) in adjacent_neighbors.iter() {
                    let x2 = x2 + x as i32;
                    let y2 = y2 + y as i32;
                    if x2 < 0 || y2 < 0 {
                        continue;
                    }
                    if x2 >= map.width as i32 || y2 >= map.height as i32 {
                        continue;
                    }
                    if let Some(symbol) = map_lookup.get(&(x2 as usize, y2 as usize)) {
                        if let EngineSymbolType::Symbol(c) = symbol.symbol_type {
                            println!("{} at ({}, {}) is adjacent to {}", num, x, y, c);
                            adjacent = true;
                            break;
                        }
                    }
                }

                if adjacent {
                    sum += num;
                    visited_ids.insert(symbol.id);
                }
            }
        }
    }

    sum
}
