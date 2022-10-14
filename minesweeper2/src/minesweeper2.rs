/// Making a wasm friendly file is trickly. only C-style enums are supported which promped the
/// creation of this second version. The main difference is how the data will be stored.
///
/// No more differences between Spot and spotType arrays. Only one 1-D array that will contain
/// the data and states
/// The difference is that all spots will have a number to indicate their state, which a vec of `u8`
/// will be wasm friendly. This makes the data more rigid since each number will refer to any of the possible states
/// a spot can be.
///
/// ## State Codes
/// - 0 - Uncovered with no neighbors
/// - 1 - Uncovered with 1 mine neighbor
/// - 2 - Uncovered with 2 mine neighbors
/// - 3 - Uncovered with 3 mine neighbors
/// - 4 - Uncovered with 4 mine neighbors
/// - 5 - Uncovered with 5 mine neighbors
/// - 6 - Uncovered with 6 mine neighbors
/// - 7 - Uncovered with 7 mine neighbors
/// - 8 - Uncovered with 8 mine neighbors
/// - 9 - Covered without a mine
/// - 10 - Covered with a mine
/// - 11 - Flagged without a mine
/// - 12 - Flagged with a mine
/// - 13 - An uncovered bomb aka Game Over
///
use wasm_bindgen::prelude::*;
extern crate web_sys;
use crate::utils::set_panic_hook;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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

const NO_MINE_COVERED: u8 = 9;
const MINE_COVERED: u8 = 10;
const NO_MINE_FLAGGED: u8 = 11;
const MINE_FLAGGED: u8 = 12;
const MINE_EXPLODED: u8 = 13;

/// this compiles only if compiling on the web and gets the js-sys crate for random number generation on the browswer
#[cfg(target_arch = "wasm32")]
fn random_number_from_range(bottom: usize, top: usize) -> usize {
    use js_sys::Math::random;
    (random() * (top - bottom) as f64) as usize + bottom
}

/// this compiles when not compiling to the web. This makes the game still playable on the cli
#[cfg(not(target_arch = "wasm32"))]
fn random_number_from_range(bottom: usize, top: usize) -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(bottom..top)
}

/// Minesweeper struct that holds the data for the game
#[wasm_bindgen]
#[derive(Debug)]
pub struct MineSweeper {
    /// map of bombs or empty spaces. Map is a one dimensional array that is calculated to index as if it was a 2d array
    game_state: Vec<u8>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl MineSweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> MineSweeper {
        set_panic_hook();
        assert!(width * height != 0, "Cannot have a height or width of zero");
        assert!(
            mine_count < width * height,
            "Cannot have more mines than spaces"
        );

        let mut game_state = vec![NO_MINE_COVERED; width * height];

        let mut mines_inserted = 0;

        while mines_inserted < mine_count {
            let random_idx = random_number_from_range(0, width * height);
            if let NO_MINE_COVERED = game_state[random_idx] {
                game_state[random_idx] = MINE_COVERED;
                mines_inserted += 1;
            }
        }

        MineSweeper {
            game_state,
            width,
            height,
        }
    }

    /// Gets a pointer to the game state for the javascript
    pub fn state(&self) -> *const u8 {
        self.game_state.as_ptr()
    }

    /// the front end will give the col and row position of the tile clicked.
    /// This is calculated into the 1-d vector and the state of the game is then updated
    /// True = Game is still going. False = game ended because of the update
    pub fn click(&mut self, col: usize, row: usize) -> bool {
        let idx = self.get_idx(col, row);
        log!(
            ">> Rust >> User clicked on col {}, row {}, of state {}",
            col,
            row,
            self.game_state[idx]
        );
        match self.game_state[idx] {
            // player clicked on an uncovered spot, empty, or flagged spot. Do nothing.
            0..=8 | MINE_FLAGGED | NO_MINE_FLAGGED => true,

            // player clicked on bomb, and this shouldn't happen but is controlled by the ui
            MINE_EXPLODED => false,

            // uncovering a tile
            MINE_COVERED | NO_MINE_COVERED => {
                if let MINE_COVERED = self.game_state[idx] {
                    self.game_state[idx] = MINE_EXPLODED;
                    return false;
                }

                let neighbor_count = self.get_mine_neighbor_count(col, row);
                if neighbor_count == 0 {
                    // allowing `uncover_empty_neighbors()` take charge of changing state to empty
                    self.uncover_empty_neighbors(col, row);
                } else {
                    self.game_state[idx] = neighbor_count;
                }
                true
            }
            _ => unreachable!(),
        }
    }

    /// The user placed a flag, usually with the right click button.
    /// Returns true if successful in placing a flag
    pub fn place_flag(&mut self, col: usize, row: usize) -> bool {
        let idx = self.get_idx(col, row);
        match self.game_state[idx] {
            MINE_COVERED => {
                self.game_state[idx] = MINE_FLAGGED;
                true
            }
            NO_MINE_COVERED => {
                self.game_state[idx] = NO_MINE_FLAGGED;
                true
            }
            _ => false,
        }
    }

    // gets the total number of bombs in the map
    pub fn get_total_bombs(&self) -> usize {
        self.game_state
            .iter()
            .filter(|&&x| x == MINE_COVERED || x == MINE_FLAGGED || x == MINE_EXPLODED)
            .count()
    }

    // finds the bombs that are hidden and not flagged
    pub fn get_hidden_bombs(&self) -> usize {
        self.game_state
            .iter()
            .filter(|&&spot| spot == MINE_COVERED)
            .count()
    }
}

impl MineSweeper {
    /// getting the total amount of bombs that are immediate neighbors to the spot passed in by col and row.
    /// max is 8, meaning all its sides and diagonals are mines
    fn get_mine_neighbor_count(&self, col: usize, row: usize) -> u8 {
        assert!(self.check_bounds(&(col as isize), &(row as isize)));
        let col = col as isize;
        let row = row as isize;

        let mine_neighbors = POSSIBLE_NEIGHBORS
            .iter()
            .filter_map(|[x, y]| self.get_idx_bounds_checked(col + x, row + y))
            .filter(|idx| {
                self.game_state[idx.to_owned()] == MINE_COVERED
                    || self.game_state[idx.to_owned()] == MINE_FLAGGED
                    || self.game_state[idx.to_owned()] == MINE_EXPLODED
            })
            .count();

        mine_neighbors as u8
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
        let state = self.game_state[idx];
        // if spot is a mine or the spot is not covered, then return early.
        if state == MINE_COVERED || state != NO_MINE_COVERED || state == MINE_EXPLODED {
            return;
        }

        let state = self.get_mine_neighbor_count(col, row);
        self.game_state[idx] = state;
        // you found a cell neighboring a bomb, so stop uncovering
        if 1 <= state && state <= 8 {
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
                let state = self.game_state[self.get_idx(col, row)];
                match state {
                    MINE_COVERED | NO_MINE_COVERED => output.push('🟩'),
                    MINE_FLAGGED | NO_MINE_FLAGGED => output.push('🚩'),
                    0 => output.push('⬜'),
                    MINE_EXPLODED => output.push('💣'),
                    1..=8 => output.push_str(&state.to_string()),
                    _ => unreachable!(),
                }
            }
            output.push('\n')
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EMPTY: u8 = 0;

    #[test]
    #[should_panic]
    fn panic_too_many_mines() {
        MineSweeper::new(10, 10, 100);
    }

    #[test]
    fn random_inserted_mines() {
        let ms = MineSweeper::new(2, 2, 2);
        assert_eq!(
            ms.game_state
                .iter()
                .filter(|&&item| item == MINE_COVERED)
                .count(),
            2
        )
    }

    #[test]
    fn test_getting_neighbors_1() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[0] = MINE_COVERED;
        assert_eq!(1, ms.get_mine_neighbor_count(0, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(1, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(1, 0));
    }

    #[test]
    fn test_getting_neighbors_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[12] = MINE_COVERED;
        assert_eq!(0, ms.get_mine_neighbor_count(0, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(2, 3));
        assert_eq!(0, ms.get_mine_neighbor_count(2, 2));
    }

    #[test]
    fn test_getting_neighbors_3() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[0] = MINE_COVERED;
        ms.game_state[1] = MINE_COVERED;
        assert_eq!(2, ms.get_mine_neighbor_count(0, 1));
        assert_eq!(2, ms.get_mine_neighbor_count(1, 1));
        assert_eq!(1, ms.get_mine_neighbor_count(2, 1));
    }

    #[test]
    #[should_panic]
    fn test_getting_neighbors_bad_input() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[4] = MINE_COVERED;
        assert_eq!(0, ms.get_mine_neighbor_count(15, 98));
    }

    #[test]
    #[should_panic]
    fn test_getting_neighbors_bad_input_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[4] = MINE_COVERED;
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
        ms.game_state[1] = MINE_COVERED;
        assert_eq!(1, ms.get_hidden_bombs());
    }

    #[test]
    fn test_get_hidden_bombs_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[1] = MINE_COVERED;
        ms.game_state[2] = MINE_COVERED;
        ms.game_state[0] = MINE_COVERED;
        ms.click(0, 0);
        assert_eq!(2, ms.get_hidden_bombs());
        assert_eq!(3, ms.get_total_bombs());
    }

    #[test]
    fn test_uncover_neighbors_1() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[0] = MINE_COVERED;
        ms.click(4, 4);

        assert_eq!(ms.game_state[ms.get_idx(4, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(0, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(4, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(0, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(4, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(0, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(4, 1)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 1)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 1)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 1)], 1);
        assert_eq!(ms.game_state[ms.get_idx(0, 1)], 1);
        assert_eq!(ms.game_state[ms.get_idx(4, 0)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 0)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 0)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 0)], 1);
        assert_eq!(ms.game_state[ms.get_idx(0, 0)], MINE_COVERED);
    }

    #[test]
    fn test_uncover_neighbors_2() {
        let mut ms = MineSweeper::new(5, 5, 0);
        ms.game_state[0] = MINE_COVERED;
        ms.game_state[1] = MINE_COVERED;
        ms.click(4, 4);

        assert_eq!(ms.game_state[ms.get_idx(4, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(0, 4)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(4, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(0, 3)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(4, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(1, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(0, 2)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(4, 1)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 1)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 1)], 1);
        assert_eq!(ms.game_state[ms.get_idx(1, 1)], 2);
        assert_eq!(ms.game_state[ms.get_idx(0, 1)], 2);
        assert_eq!(ms.game_state[ms.get_idx(4, 0)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(3, 0)], EMPTY);
        assert_eq!(ms.game_state[ms.get_idx(2, 0)], 1);
        assert_eq!(ms.game_state[ms.get_idx(1, 0)], MINE_COVERED);
        assert_eq!(ms.game_state[ms.get_idx(0, 0)], MINE_COVERED);
    }
}
