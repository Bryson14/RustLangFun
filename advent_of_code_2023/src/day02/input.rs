#[derive(Debug, PartialEq)]
pub struct GameSet {
    green: u32,
    red: u32,
    blue: u32,
}

impl GameSet {
    /// Returns true if the set is possible given the max number of cubes of each color
    pub fn is_possible(&self, red_max: u32, green_max: u32, blue_max: u32) -> bool {
        self.red <= red_max && self.green <= green_max && self.blue <= blue_max
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    /// Returns true if the game is possible given the max number of cubes of each color
    pub fn is_possible(&self, red_max: u32, green_max: u32, blue_max: u32) -> bool {
        self.sets
            .iter()
            .all(|set| set.is_possible(red_max, green_max, blue_max))
    }

    pub fn get_min_power_required(&self) -> u32 {
        // get the max number of cubes of each color
        let mut red_max: u32 = 0;
        let mut green_max: u32 = 0;
        let mut blue_max: u32 = 0;

        for set in &self.sets {
            if set.red > red_max {
                red_max = set.red;
            }
            if set.green > green_max {
                green_max = set.green;
            }
            if set.blue > blue_max {
                blue_max = set.blue;
            }
        }

        // return the 'power' of the max set
        red_max * green_max * blue_max
    }
}

/// Gets the game id from the input string
/// Example:
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// should return 1
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// should return 2
/// Read from the front of the string until you hit a colon
pub fn get_game_id(input: &str) -> u32 {
    let mut game_id_str: String = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            game_id_str.push(c);
        } else if c == ':' {
            break;
        }
    }

    game_id_str.parse::<u32>().unwrap()
}

/// Parses a single set from the input string
/// Example:
/// 3 blue, 4 red
/// should return a GameSet struct with green=3, red=4, blue=0
/// 10 red, 2 green, 6 blue
/// should return a GameSet struct with green=2, red=10, blue=6
fn parse_game_set(input: &str) -> GameSet {
    let mut green: u32 = 0;
    let mut red: u32 = 0;
    let mut blue: u32 = 0;

    // split the input string by ',' to get the individual cubes
    let cubes_split = input.split(',');

    // parse each cube into a GameSet struct
    for cube in cubes_split {
        let cube_split: Vec<&str> = cube.trim().split(' ').collect();
        let num = cube_split[0].parse::<u32>().unwrap();
        let color = cube_split[1];

        match color {
            "green" => green = num,
            "red" => red = num,
            "blue" => blue = num,
            _ => panic!("Invalid color: {}", color),
        }
    }

    GameSet { green, red, blue }
}

/// Parses a single input line into a Game struct
/// Example:
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// should return a Game struct with id=1 and sets=[GameSet{green=3, red=4, blue=0}, GameSet{green=2, red=1, blue=6}]
pub fn parse_game(input: &str) -> Game {
    let game_id = get_game_id(input);

    // split the input string by ':' to get rid of the game id
    let mut input_split = input.split(':');

    // split the remaining string by ';' to get the sets
    let sets_split = input_split.nth(1).unwrap().split(';');

    // parse each set into a GameSet struct
    let sets: Vec<GameSet> = sets_split.map(parse_game_set).collect();

    Game { id: game_id, sets }
}

/// Parses the entire input file into a vector of Game structs
pub fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_game_id() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = 1;
        let actual = get_game_id(input);
        assert_eq!(expected, actual);

        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = 2;
        let actual = get_game_id(input);
        assert_eq!(expected, actual);

        let input = "Game 34: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = 34;
        let actual = get_game_id(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_game_set() {
        let input = "3 blue, 4 red";
        let expected = GameSet {
            green: 0,
            red: 4,
            blue: 3,
        };
        let actual = parse_game_set(input);
        assert_eq!(expected, actual);

        let input = "1 red, 2 green, 6 blue";
        let expected = GameSet {
            green: 2,
            red: 1,
            blue: 6,
        };
        let actual = parse_game_set(input);
        assert_eq!(expected, actual);

        let input = "5 red, 13 blue, 7 green";
        let expected = GameSet {
            green: 7,
            red: 5,
            blue: 13,
        };
        let actual = parse_game_set(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            id: 1,
            sets: vec![
                GameSet {
                    green: 0,
                    red: 4,
                    blue: 3,
                },
                GameSet {
                    green: 2,
                    red: 1,
                    blue: 6,
                },
                GameSet {
                    green: 2,
                    red: 0,
                    blue: 0,
                },
            ],
        };
        let actual = parse_game(input);
        assert_eq!(expected, actual);

        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = Game {
            id: 2,
            sets: vec![
                GameSet {
                    green: 2,
                    red: 0,
                    blue: 1,
                },
                GameSet {
                    green: 3,
                    red: 1,
                    blue: 4,
                },
                GameSet {
                    green: 1,
                    red: 0,
                    blue: 1,
                },
            ],
        };
        let actual = parse_game(input);
        assert_eq!(expected, actual);
    }
}
