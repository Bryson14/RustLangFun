// store 2 battle ship maps
// print out maps either seen or just showing where the hits and misses are
// take user input to setup (maybe just make a file)
// take user input as a missel every turn
// track location of missles
// track ships remaining

use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};

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

    pub fn missle_strike(&mut self, col: usize, row: char) -> Result<(String), String> {
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
        return match self.map[idx] {
            BattleshipLocation::Empty => {
                self.map[idx] = BattleshipLocation::Miss;
                Ok(String::from("Miss"))
            }
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
                    Ok(format!("Sunk the ship {}!", ship.id))
                } else {
                    Ok(String::from("Hit"))
                }
            }
            BattleshipLocation::HitShip(_ship) => {
                Err("You have already shot and hit this ship at this location".to_string())
            }
            BattleshipLocation::Miss => {
                Err("You have already shot and missed at this location".to_string())
            }
            BattleshipLocation::DestroyedShip(_ship) => {
                Err("You have already shot and sunk this ship".to_string())
            }
        };
    }

    fn all_destroyed(&self) -> bool {
        self.map.iter().all(|x| match x {
            BattleshipLocation::OkShip(_) => false,
            _ => true,
        })
    }

    fn get_index(&self, col: usize, row: usize) -> usize {
        self.width * row + col
    }

    fn print_map(&self, private: bool) {
        // println!("Map - Top Secret?: {}", private);
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

fn get_user_input(message: &str) -> (usize, char) {
    let mut s = String::new();
    println!("{}", message);
    print!("Please tell me where to launch the missle! e.g. 'D5'.\n> ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    // getting rid of newline
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    let characters: Vec<char> = s.chars().collect();
    if characters.len() != 2 {
        println!("Please try again entering coordinates. You entered {}", s);
        return get_user_input(message);
    } else {
        let row = characters[0];
        let col: usize = match characters[1].to_digit(10) {
            Some(num) => num as usize,
            None => {
                println!("Err parsing letter. You entered {}", s);
                return get_user_input(message);
            }
        };
        (col, row)
    }
}

fn players_turn<'a>(player: usize, map: &'a mut Map) {
    'inner: loop {
        println!("PLAYER {}'s TURN", player);
        map.print_map(false);
        let (col, row) =
            get_user_input(&format!("Player {}, choose a location to strike!", player));
        match map.missle_strike(col, row) {
            Ok(message) if message.contains("Hit") => {
                println!("{}", message);
                println!("Player {}, GO Again", player);
            }
            Ok(message) if message.contains("Sunk") => {
                println!("{}", message);
                if map.all_destroyed() {
                    break 'inner;
                } else {
                    println!("Player {}, GO Again", player);
                }
            }
            Ok(message) if message.contains("Miss") => {
                println!("{}", message);
                break 'inner;
            }
            Ok(message) => {
                println!("Unknown message: {}", message);
                break 'inner;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn take_turns() {
    use std::thread::sleep;
    use std::time::Duration;
    println!("\n++++++++++++++\nChanging Turns!\n++++++++++++++\n");
    sleep(Duration::from_millis(1500));
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

    let winner: u8 = 'outer: loop {
        players_turn(2, &mut player_one_map);
        if player_one_map.all_destroyed() {
            break 'outer 2;
        }
        take_turns();

        players_turn(1, &mut player_two_map);
        if player_two_map.all_destroyed() {
            break 'outer 1;
        }
        take_turns();
    };

    println!("Congratulations! Player {} won!", winner);
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
