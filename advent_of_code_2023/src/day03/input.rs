use std::collections::{HashMap, HashSet};

pub fn read_to_map(input: &str) -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        width = row.len();
        map.push(row);
        height += 1;
    }

    Map { map, width, height }
}

pub struct Map {
    pub map: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq)]
pub enum EngineSymbolType {
    Number(u32), // this will be th entire number, not just the digit
    Empty,
    Symbol(char),
}

#[derive(Debug, PartialEq)]
pub struct EngineSymbol {
    pub symbol_type: EngineSymbolType,
    pub id: usize, // for grouping Number() with different positions and the same value
}

/// Reads over the map and creates a hashmap of EngineSymbols
/// This is because number can span multiple positions
pub fn get_engine_symbol_map(map: &Map) -> HashMap<(usize, usize), EngineSymbol> {
    let mut engine_symbols: HashMap<(usize, usize), EngineSymbol> = HashMap::new();
    let mut id = 0;

    for y in 0..map.height {
        let mut visited: HashSet<u32> = HashSet::new();
        for x in 0..map.width {
            // check if we've already visited this x coordinate
            // while gathering a digit
            if visited.contains(&(x as u32)) {
                continue;
            }

            let c = map.map[y][x];
            // check if c is a digit
            if c.is_ascii() {
                let num: u32 = c.to_digit(10).unwrap();
                let mut x2 = x + 1;
                let mut digits: Vec<u32> = vec![num];

                // check if the next character is a digit
                while x2 < map.width {
                    let c2 = map.map[y][x2];
                    if c2.is_ascii_digit() {
                        // add the neighboring x coordinate to the visited set
                        visited.insert(x2 as u32);
                        let num = c2.to_digit(10).unwrap();
                        digits.push(num);
                        x2 += 1;
                    } else {
                        break;
                    }
                }
                // convert the digits vector into a single number
                let num = digits.iter().fold(0, |acc, x| acc * 10 + x);
                // iterate through the digits and add them to the hashmap
                for (i, _num) in digits.iter().enumerate() {
                    engine_symbols.insert(
                        (x + i, y),
                        EngineSymbol {
                            symbol_type: EngineSymbolType::Number(num),
                            id,
                        },
                    );
                }
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Number(num),
                        id,
                    },
                );
            } else if c == '.' {
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Empty,
                        id,
                    },
                );
            } else {
                engine_symbols.insert(
                    (x, y),
                    EngineSymbol {
                        symbol_type: EngineSymbolType::Symbol(c),
                        id,
                    },
                );
            }
            id += 1;
        }
    }

    engine_symbols
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_to_map() {
        let input = "467.\n\
                     8...\n\
                     32.1\n\
                     ....";
        let map = read_to_map(input);
        assert_eq!(map.width, 4);
        assert_eq!(map.height, 4);
        assert_eq!(map.map[0][0], '4');
        assert_eq!(map.map[0][1], '6');
        assert_eq!(map.map[0][2], '7');
        assert_eq!(map.map[0][3], '.');
        assert_eq!(map.map[1][0], '8');
        assert_eq!(map.map[1][1], '.');
        assert_eq!(map.map[1][2], '.');
        assert_eq!(map.map[1][3], '.');
        assert_eq!(map.map[2][0], '3');
        assert_eq!(map.map[2][1], '2');
        assert_eq!(map.map[2][2], '.');
        assert_eq!(map.map[2][3], '1');
        assert_eq!(map.map[3][0], '.');
        assert_eq!(map.map[3][1], '.');
        assert_eq!(map.map[3][2], '.');
        assert_eq!(map.map[3][3], '.');
    }

    #[test]
    fn test_get_engine_symbol_map() {
        let input = "467.\n\
            ...*\n\
            ..35\n";

        let map = read_to_map(input);
        let engine_symbols = get_engine_symbol_map(&map);

        let mut expected: HashMap<(usize, usize), EngineSymbol> = HashMap::new();
        expected.insert(
            (0, 0),
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(467),
                id: 0,
            },
        );
        expected.insert(
            (1, 0),
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(467),
                id: 0,
            },
        );
        expected.insert(
            (2, 0),
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(467),
                id: 0,
            },
        );

        expected.insert(
            (3, 0),
            EngineSymbol {
                symbol_type: EngineSymbolType::Empty,
                id: 1,
            },
        );
        expected.insert(
            (0, 1),
            EngineSymbol {
                symbol_type: EngineSymbolType::Empty,
                id: 2,
            },
        );
        expected.insert(
            (1, 1),
            EngineSymbol {
                symbol_type: EngineSymbolType::Empty,
                id: 3,
            },
        );
        expected.insert(
            (2, 1),
            EngineSymbol {
                symbol_type: EngineSymbolType::Empty,
                id: 4,
            },
        );
        expected.insert(
            (3, 1),
            EngineSymbol {
                symbol_type: EngineSymbolType::Symbol('*'),
                id: 5,
            },
        );
        expected.insert(
            (0, 2),
            EngineSymbol {
                symbol_type: EngineSymbolType::Empty,
                id: 6,
            },
        );
        expected.insert(
            (1, 2),
            EngineSymbol {
                symbol_type: EngineSymbolType::Empty,
                id: 7,
            },
        );
        expected.insert(
            (2, 2),
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(35),
                id: 8,
            },
        );
        expected.insert(
            (3, 2),
            EngineSymbol {
                symbol_type: EngineSymbolType::Number(35),
                id: 8,
            },
        );

        assert_eq!(engine_symbols, expected);
    }
}
