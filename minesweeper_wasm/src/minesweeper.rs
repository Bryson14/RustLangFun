//! # Minesweeper
//! This is a project to make a minesweeper copy that is used in wasm and ran on the browswer
//! I'm excited to see what happens

extern crate js_sys;
extern crate rand;

use rand::thread_rng;
use rand::Rng;

const POSSIBLE_NEIGHBORS: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn random_number_from_range(bottom: usize, top: usize) -> usize {
    js_sys::Math.random() * high + bottom
}

#[cfg(not(target_arch = "wasm32", target_os = "unknown"))]
fn random_number_from_range(bottom: usize, top: usize) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(bottom..top)
}

/// Minesweeper struct that holds the data for the game
#[wasm_bindgen]
pub struct MineSweeper {
    /// map of bombs or empty spaces. Map is a one dimensional array that is calculated to index as if it was a 2d array
    mine_vec: Vec<Spot>,
    /// map of the state of each tile.
    state_vec: Vec<SpotState>,
    width: usize,
    height: usize,
}

impl MineSweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> MineSweeper {
        assert!(width * height != 0, "Cannot have a height or width of zero");
        assert!(
            mine_count < width * height,
            "Cannot have more mines than spaces"
        );

        let mut mine_vec: Vec<Spot> = vec![Spot::Safe; width * height];

        let mut mines_inserted = 0;

        while mines_inserted < mine_count {
            let random_idx = random_number_from_range(0, width * height);
            if let Spot::Safe = mine_vec[random_idx] {
                mine_vec[random_idx] = Spot::Mine;
                mines_inserted += 1;
            }
        }

        let state_vec: Vec<SpotState> = vec![SpotState::Covered; width * height];

        MineSweeper {
            mine_vec,
            state_vec,
            width,
            height,
        }
    }

    /// the front end will give the col and row position of the tile clicked.
    /// This is calculated into the 1-d vector and the state of the game is then updated
    /// True = Game is still going. False = game ended because of the update
    pub fn click(&mut self, col: usize, row: usize) -> bool {
        let idx = self.get_idx(col, row);
        match self.state_vec[idx] {
            // player clicked on an uncovered spot, empty, or flagged spot. Do nothing.
            SpotState::Flagged | SpotState::Empty | SpotState::Numbered(_) => true,

            // player clicked on bomb, and this shouldn't happen but is controlled by the ui
            SpotState::Exploded => false,

            // uncovering a tile
            SpotState::Covered => {
                if let Spot::Mine = self.mine_vec[idx] {
                    self.state_vec[idx] = SpotState::Exploded;
                    return false;
                }

                let neighbor_count = self.get_mine_neighbor_count(col, row);
                if neighbor_count == 0 {
                    // allowing `uncover_empty_neighbors()` take charge of changing state to empty
                    self.uncover_empty_neighbors(col, row);
                } else {
                    self.state_vec[idx] = SpotState::Numbered(neighbor_count);
                }
                true
            }
        }
    }

    /// The user placed a flag, usually with the right click button.
    /// Returns true if successful in placing a flag
    pub fn place_flag(&mut self, col: usize, row: usize) -> bool {
        let idx = self.get_idx(col, row);
        match self.state_vec[idx] {
            SpotState::Covered => {
                self.state_vec[idx] = SpotState::Flagged;
                true
            }
            _ => false,
        }
    }

    /// assuming zero-indexed grid
    /// 0,0 0,1 0,2
    /// 1,0 1,1 1,2
    /// 2,0 2,1 2,2
    fn get_idx(&self, col: usize, row: usize) -> usize {
        self.width * row + col
    }

    fn get_idx_bounds_checked(&self, col: isize, row: isize) -> Option<usize> {
        if !self.check_bounds(&col, &row) {
            return None;
        }

        Some(self.get_idx(col as usize, row as usize))
    }

    /// for uncovering empty neighbors and recursively uncovering their empty neighbors
    fn uncover_empty_neighbors(&mut self, col: usize, row: usize) {
        // break case for recursion

        let idx = self.get_idx(col, row);
        let spot_state = self.state_vec[idx];
        let mine_state = self.mine_vec[idx];
        // if spot is a mine or the spot is not covered, then return early.
        if spot_state != SpotState::Covered || mine_state == Spot::Mine {
            return;
        }

        let mine_neighbors = self.get_mine_neighbor_count(col, row);

        if mine_neighbors == 0 {
            self.state_vec[idx] = SpotState::Empty;
        } else {
            self.state_vec[idx] = SpotState::Numbered(mine_neighbors);
            return;
        }

        // copying height and width so that for_each can borrow self in the closure.
        let (width, height) = (self.width, self.height);
        let _inbound_neighbors = POSSIBLE_NEIGHBORS
            .iter()
            .map(|[x, y]| [x + col as isize, y + row as isize])
            .filter(|[x, y]| check_bound_usize(*x, *y, width, height))
            .for_each(|[x, y]| self.uncover_empty_neighbors(x as usize, y as usize));
    }

    /// for checking the bounds of a given row and col since neighbors are blindly checked.
    fn check_bounds(&self, col: &isize, row: &isize) -> bool {
        if col < &0 || row < &0 || col >= &(self.width as isize) || row >= &(self.height as isize) {
            return false;
        }
        true
    }

    /// getting the total amount of bombs that are immediate neighbors to the spot passed in by col and row.
    /// max is 8, meaning all its sides and diagonals are mines
    fn get_mine_neighbor_count(&self, col: usize, row: usize) -> usize {
        assert!(self.check_bounds(&(col as isize), &(row as isize)));
        let col = col as isize;
        let row = row as isize;

        let mine_neighbors = POSSIBLE_NEIGHBORS
            .iter()
            .filter_map(|[x, y]| self.get_idx_bounds_checked(col + x, row + y))
            .filter(|idx| self.mine_vec[idx.to_owned()] == Spot::Mine)
            .count();

        mine_neighbors
    }

    // gets the total number of bombs in the map
    pub fn get_total_bombs(&self) -> usize {
        self.mine_vec.iter().filter(|&&x| x == Spot::Mine).count()
    }

    // finds the bombs that are hidden and not flagged
    pub fn get_hidden_bombs(&self) -> usize {
        self.mine_vec
            .iter()
            .enumerate()
            .filter(|(idx, spot)| {
                **spot == Spot::Mine && self.state_vec[*idx] == SpotState::Covered
            })
            .count()
    }
}

// checks if the given row and col are in bounds with usizes.
fn check_bound_usize(col: isize, row: isize, width: usize, height: usize) -> bool {
    if col < 0 || row < 0 || col >= (width as isize) || row >= (height as isize) {
        return false;
    }
    true
}

impl std::fmt::Display for MineSweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        output.push('\n');
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_idx(col, row);
                match self.state_vec[idx] {
                    SpotState::Covered => output.push('🟩'),
                    SpotState::Flagged => output.push('🚩'),
                    SpotState::Empty => output.push('⬜'),
                    SpotState::Exploded => output.push('💣'),
                    SpotState::Numbered(number) => output.push_str(&format!(" {}", number)),
                }
            }
            output.push('\n')
        }
        write!(f, "{}", output)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Spot {
    Mine,
    Safe,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum SpotState {
    Covered,         // covered tile
    Flagged,         // covered tile with flag on top. Cannot be accidentally clicked
    Empty,           // uncovered, empty spot
    Exploded,        // a formality for showing mines at the end, not during game play
    Numbered(usize), // has a number underneath to show how many of its neighbors are mines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_too_many_mines() {
        MineSweeper::new(10, 10, 100);
    }

    #[test]
    fn test_getting_neighbors_1() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[0] = Spot::Mine;
        assert_eq!(1, ms.get_mine_neighbor_count(0, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(1, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(1, 0));
    }

    #[test]
    fn test_getting_neighbors_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[12] = Spot::Mine;
        assert_eq!(0, ms.get_mine_neighbor_count(0, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(2, 3));
        assert_eq!(0, ms.get_mine_neighbor_count(2, 2));
    }

    #[test]
    fn test_getting_neighbors_3() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[0] = Spot::Mine;
        ms.mine_vec[1] = Spot::Mine;
        assert_eq!(2, ms.get_mine_neighbor_count(0, 1));
        assert_eq!(2, ms.get_mine_neighbor_count(1, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(2, 1));
    }

    #[test]
    #[should_panic]
    fn test_getting_neighbors_bad_input() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[4] = Spot::Mine;
        assert_eq!(0, ms.get_mine_neighbor_count(15, 98));
    }

    #[test]
    #[should_panic]
    fn test_getting_neighbors_bad_input_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[4] = Spot::Mine;
        assert_eq!(1, ms.get_mine_neighbor_count(5, 0));
    }

    #[test]
    fn test_get_total_bomb() {
        let ms = MineSweeper::new(5, 5, 7);
        assert_eq!(7, ms.get_total_bombs());
    }

    #[test]
    fn test_get_hidden_bombs() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[1] = Spot::Mine;
        assert_eq!(1, ms.get_hidden_bombs());
    }

    #[test]
    fn test_get_hidden_bombs_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[1] = Spot::Mine;
        ms.mine_vec[2] = Spot::Mine;
        ms.mine_vec[0] = Spot::Mine;
        ms.click(0, 0);
        assert_eq!(2, ms.get_hidden_bombs());
        assert_eq!(3, ms.get_total_bombs());
    }

    #[test]
    fn test_uncover_neighbors_1() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[0] = Spot::Mine;
        assert_eq!(ms.state_vec, vec![SpotState::Covered; 25]);
        ms.click(4, 4);

        assert_eq!(ms.state_vec[ms.get_idx(4, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(0, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(4, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(0, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(4, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(0, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(4, 1)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 1)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 1)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 1)], SpotState::Numbered(1));
        assert_eq!(ms.state_vec[ms.get_idx(0, 1)], SpotState::Numbered(1));
        assert_eq!(ms.state_vec[ms.get_idx(4, 0)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 0)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 0)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 0)], SpotState::Numbered(1));
        assert_eq!(ms.state_vec[ms.get_idx(0, 0)], SpotState::Covered);
    }

    #[test]
    fn test_uncover_neighbors_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.mine_vec[0] = Spot::Mine;
        ms.mine_vec[1] = Spot::Mine;
        assert_eq!(ms.state_vec, vec![SpotState::Covered; 25]);
        ms.click(4, 4);

        assert_eq!(ms.state_vec[ms.get_idx(4, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(0, 4)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(4, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(0, 3)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(4, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(1, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(0, 2)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(4, 1)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 1)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 1)], SpotState::Numbered(1));
        assert_eq!(ms.state_vec[ms.get_idx(1, 1)], SpotState::Numbered(2));
        assert_eq!(ms.state_vec[ms.get_idx(0, 1)], SpotState::Numbered(2));
        assert_eq!(ms.state_vec[ms.get_idx(4, 0)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(3, 0)], SpotState::Empty);
        assert_eq!(ms.state_vec[ms.get_idx(2, 0)], SpotState::Numbered(1));
        assert_eq!(ms.state_vec[ms.get_idx(1, 0)], SpotState::Covered);
        assert_eq!(ms.state_vec[ms.get_idx(0, 0)], SpotState::Covered);
    }
}
