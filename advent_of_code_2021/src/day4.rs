use crate::read_from_data_dir;

/// --- Day 4: Giant Squid ---
/// You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.
///
/// Maybe it wants to play bingo?
///
/// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)
///
/// The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:
/// ```text
/// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
///
/// 22 13 17 11  0
///  8  2 23  4 24
/// 21  9 14 16  7
///  6 10  3 18  5
///  1 12 20 15 19
///
///  3 15  0  2 22
///  9 18 13 17  5
/// 19  8  7 25 23
/// 20 11 10 24  4
/// 14 21 16 12  6
///
/// 14 21 17 24  4
/// 10 16 15  9 19
/// 18  8 23 26 20
/// 22 11 13  6  5
///  2  0 12  3  7
/// ```
/// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):
/// ```text
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
/// ```
/// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
/// ```text
/// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
///  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
/// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
///  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
///  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
/// ```
/// Finally, 24 is drawn:
/// At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: `14 21 17 24 4`).
///
/// The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.
///
/// To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
pub fn part1() {
    let game_info = read_from_data_dir("day4.txt").unwrap();
    let (instructions, bingo_boards) = parse_game_info(game_info);

    // play the game
    let mut called_nums: Vec<u8> = Vec::new();
    'outer: for num in instructions {
        called_nums.push(num);

        for board in &bingo_boards {
            match board.winning_line(&called_nums) {
                Some(v) => {
                    println!("winner: {:?}", v);
                    break 'outer;
                }
                None => (),
            }
        }
    }
}

fn parse_game_info(s: String) -> (Vec<u8>, Vec<BingoBoard>) {
    (vec![0], vec![BingoBoard::new()])
}

struct BingoBoard {
    board: [[u8; 5]; 5],
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard { board: [[0; 5]; 5] }
    }

    fn winning_line(&self, called_nums: &Vec<u8>) -> Option<[u8; 5]> {
        let mut all_called = true;
        let mut winning_col = [0u8; 5];

        for col in 0..self.board[0].len() {
            // check rows and cols
            for row in self.board {
                if row.iter().any(|x| !called_nums.contains(&x)) {
                    println!("The right row: {:?}", row);
                    return Some(row);
                }

                // checking cols
                if called_nums.contains(&row[col]) {
                    winning_col[col] = row[col];
                } else {
                    all_called = false;
                }
            }

            if all_called {
                println!("the right col: {:?}", winning_col);
                return Some(winning_col);
            }
        }

        // checking diagonal
        let mut right_diagonal = true;
        let mut right_diagonal_nums = [0u8; 5];
        let mut left_diagonal = true;
        let mut left_diagonal_nums = [0u8; 5];
        for i in 0..self.board[0].len() {
            left_diagonal_nums[i] = self.board[i][i];
            if !called_nums.contains(&self.board[i][i]) {
                left_diagonal = false;
            }
            let vert = self.board.len() - 1 - i;
            right_diagonal_nums[i] = self.board[i][vert];
            if !called_nums.contains(&self.board[i][vert]) {
                right_diagonal = false;
            }
        }

        if left_diagonal {
            println!("Left diag: {:?}", left_diagonal_nums);
            return Some(left_diagonal_nums);
        }

        if right_diagonal {
            println!("right diag: {:?}", right_diagonal_nums);
            return Some(right_diagonal_nums);
        }

        None
    }
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_info() {}

    #[test]
    fn test_movement_with_aim() {}
}
