// store 2 battle ship maps
// print out maps either seen or just showing where the hits and misses are
// take user input to setup (maybe just make a file)
// take user input as a missel every turn
// track location of missles
// track ships remaining

use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Ship<'a> {
    id: u8,
    name: &'a str,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum BattleshipLocation<'a> {
    Empty,
    OkShip(Ship<'a>),
    HitShip(Ship<'a>),
    DestroyedShip(Ship<'a>),
    Miss,
}

#[derive(Debug)]
struct Map<'a> {
    height: usize,
    width: usize,
    map: Vec<BattleshipLocation<'a>>,
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        let h = 10;
        let w = 10;
        let m = vec![BattleshipLocation::Empty; h * w];
        Map {
            height: h,
            width: w,
            map: m,
        }
    }

    fn read_map_config(&mut self, filename: &str) -> Result<(), String> {
        let contents = open_file(filename).unwrap();
        let mut map: Vec<BattleshipLocation> = Vec::new();
        contents.split("\n").for_each(|line| {
            line.split(" ").for_each(|c| {
                let c = &c[0..1];
                let loc: BattleshipLocation = match c {
                    "." => BattleshipLocation::Empty,
                    "1" => BattleshipLocation::OkShip(Ship {
                        id: 1,
                        name: "Destroyer",
                    }),
                    "2" => BattleshipLocation::OkShip(Ship {
                        id: 2,
                        name: "Submarine",
                    }),
                    "3" => BattleshipLocation::OkShip(Ship {
                        id: 3,
                        name: "Cruiser",
                    }),
                    "4" => BattleshipLocation::OkShip(Ship {
                        id: 4,
                        name: "Battle Ship",
                    }),
                    "5" => BattleshipLocation::OkShip(Ship {
                        id: 5,
                        name: "Carrier",
                    }),
                    _ => {
                        println!("Strange character: {}", c);
                        BattleshipLocation::Empty
                    }
                };
                map.push(loc);
            })
        });

        if map.len() != 100 {
            return Err(format!(
                "The size of the map is not 10 x 10.\nMap: {:?}",
                map
            ));
        }

        self.map = map;
        Ok(())
    }

    pub fn missle_strike(&mut self, col: usize, row: char) -> Result<(), String> {
        let row_idx = match row {
            'A' | 'a' => 0,
            'B' | 'b' => 1,
            'C' | 'c' => 2,
            'D' | 'd' => 3,
            'E' | 'e' => 4,
            'F' | 'f' => 5,
            'G' | 'g' => 6,
            'H' | 'h' => 7,
            'I' | 'i' => 8,
            'J' | 'j' => 9,
            _ => {
                return Err(format!(
                    "Unknown character '{}' put into missle strike function",
                    row
                ))
            }
        };

        if row_idx >= self.height || col >= self.width {
            return Err("row_idx or col is larger than map width".to_string());
        }

        let idx = self.get_index(col, row_idx);
        match self.map[idx] {
            BattleshipLocation::Empty => self.map[idx] = BattleshipLocation::Miss,
            BattleshipLocation::OkShip(ship) => {
                self.map[idx] = BattleshipLocation::HitShip(ship);
                if !self.map.contains(&BattleshipLocation::OkShip(ship)) {
                    // there is not okay part of the ship left
                    self.map = self
                        .map
                        .iter_mut()
                        .map(|&mut x| {
                            if x == BattleshipLocation::HitShip(ship) {
                                BattleshipLocation::DestroyedShip(ship)
                            } else {
                                x
                            }
                        })
                        .collect::<Vec<BattleshipLocation>>();
                }
            }
            BattleshipLocation::HitShip(_ship) => {
                return Err("You have already shot and hit this ship at this location".to_string())
            }
            BattleshipLocation::Miss => {
                return Err("You have already shot and missed at this location".to_string())
            }
            BattleshipLocation::DestroyedShip(_ship) => {
                return Err("You have already shot and sunk this ship".to_string())
            }
        }

        Ok(())
    }

    fn get_index(&self, col: usize, row: usize) -> usize {
        self.width * row + col
    }

    fn print_map(&self, private: bool) {
        println!("Map - Top Secret?: {}", private);
        let row_names = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
        print!("   ");
        for x in 0..self.width {
            print!("{} ", x);
        }
        println!();
        print!("  ");
        for _ in 0..self.width {
            print!("__");
        }
        println!();
        for row in 0..self.height {
            print!("{}| ", row_names[row]);
            for col in 0..self.width {
                let location = self.map[self.get_index(col, row)];
                let to_print = match location {
                    BattleshipLocation::Miss => "o".to_string(),
                    BattleshipLocation::DestroyedShip(_) => "X".to_string(),
                    BattleshipLocation::HitShip(_) => "x".to_string(),
                    BattleshipLocation::Empty => ".".to_string(),
                    BattleshipLocation::OkShip(ship) => {
                        if private {
                            ship.id.to_string()
                        } else {
                            ".".to_string()
                        }
                    }
                };
                print!("{} ", to_print)
            }
            println!();
        }
        println!();
    }
}

fn open_file(filename: &str) -> Result<String, String> {
    let curr_exe = env::current_exe().unwrap();
    let curr_dir = curr_exe
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let file_loc = curr_dir.join("data").join(filename);
    println!("file: {:?}", file_loc);

    let mut file = File::open(file_loc).expect("Cannot find the file in data directory");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

pub fn play_battleship() {
    println!("Yeah battleship");
    let mut player_one_map = Map::new();
    let mut player_two_map = Map::new();

    player_one_map
        .read_map_config("player_1_battleship.txt")
        .unwrap();
    player_two_map
        .read_map_config("player_2_battleship.txt")
        .unwrap();

    player_two_map.missle_strike(1, 'A').unwrap();
    player_two_map.missle_strike(2, 'A').unwrap();
    player_two_map.print_map(false);
    player_two_map.missle_strike(3, 'A').unwrap();

    player_two_map.print_map(true);

    loop {
        // get input from player 1
        // missle strike
        // check map 2 if all destroyed

        // get input from player 2
        // missle strike
        // check map 1 if all destroyed
        break;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let map = Map::new();

        assert_eq!(map.map[0], BattleshipLocation::Empty);
        assert_eq!(map.get_index(2, 2), 22 as usize);
    }

    #[test]
    fn test_map_bad_missle_strike() {
        let mut map = Map::new();

        assert_eq!(
            map.missle_strike(10, 'A'),
            Err("row_idx or col is larger than map width".to_string())
        );

        assert_eq!(
            map.missle_strike(15, 'a'),
            Err("row_idx or col is larger than map width".to_string())
        );

        assert_eq!(
            map.missle_strike(10, 'Y'),
            Err("Unknown character 'Y' put into missle strike function".to_string())
        );
    }
}
