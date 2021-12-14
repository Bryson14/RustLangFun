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
                Some((winning_line, sum_unused)) => {
                    println!(
                        "Day4:1 Answer is {}. Winning line was {:?}",
                        sum_unused * num as i32,
                        winning_line
                    );
                    break 'outer;
                }
                None => (),
            }
        }
    }
}

/// Takes the text input and turns it into bingo boards and instrcutions
/// The text is guarenteed to follow the format:
/// ```text
/// 1,2,3,4,5,6.... (instructions)
///
/// 9 54 32 10 7
/// 98 12 4 7 6
/// 3 5 12 6 4
/// 84 5 12 11 3
/// 9 65 32 82 6
///
/// ... (more game boards)
/// ```
fn parse_game_info(s: String) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut lines = s.lines();
    let instructions: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|num| num.trim().parse().expect("Bad Number"))
        .collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut board = BingoBoard::new();
    let mut current_row = 0;

    for line in lines {
        if line.len() < 4 {
            continue;
        }
        let row: Vec<u8> = line
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| s.parse().expect("Bad Number"))
            .collect();

        for i in 0..5 {
            board.board[current_row][i] = row[i];
        }
        current_row += 1;

        if current_row >= 5 {
            current_row = 0;
            boards.push(board.clone());
            board = BingoBoard::new();
        }
    }

    (instructions, boards)
}

#[derive(Clone, Debug, PartialEq)]
struct BingoBoard {
    board: [[u8; 5]; 5],
}

/// A bingo board represented as a 5x5 array. It has a function call winning_line
/// that checks if the board has won based on the called numbers, then returns
/// the winning line that won in the array.
impl BingoBoard {
    fn new() -> Self {
        BingoBoard { board: [[0; 5]; 5] }
    }

    // returns the winning line and the sum of the unused numbers
    fn winning_line(&self, called_nums: &Vec<u8>) -> Option<([u8; 5], i32)> {
        // checking rows
        for row in self.board {
            if row.iter().all(|x| called_nums.contains(&x)) {
                println!("Winning board: {:?}", self.board);
                return Some((row, self.sum_of_unused_numbers(row)));
            }
        }

        for col in 0..self.board[0].len() {
            let mut winning_col: [u8; 5] = [0; 5];
            let mut all_called = true;
            for row in 0..self.board.len() {
                if called_nums.contains(&self.board[row][col]) {
                    winning_col[row] = self.board[row][col];
                } else {
                    all_called = false;
                }
            }

            if all_called {
                println!("Winning board: {:?}", self.board);
                return Some((winning_col, self.sum_of_unused_numbers(winning_col)));
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
            println!("Winning board: {:?}", self.board);
            return Some((
                left_diagonal_nums,
                self.sum_of_unused_numbers(left_diagonal_nums),
            ));
        }

        if right_diagonal {
            println!("Winning board: {:?}", self.board);
            return Some((
                right_diagonal_nums,
                self.sum_of_unused_numbers(right_diagonal_nums),
            ));
        }

        None
    }

    fn sum_of_unused_numbers(&self, winning_numbers: [u8; 5]) -> i32 {
        let mut total: i32 = 0;
        for row in self.board.iter() {
            total += row
                .iter()
                .filter(|x| !winning_numbers.contains(x))
                .map(|&n| n as i32)
                .sum::<i32>()
        }
        total
    }
}

pub fn part2() {}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winning_line() {
        let mut board = BingoBoard::new();
        for i in 0..=4 {
            board.board[0][i] = (i + 1) as u8;
        }
        println!("board: {:?}", board.board);
        let called_nums = vec![1, 2, 3, 4, 5];
        assert_eq!(
            board.winning_line(&called_nums).unwrap().0,
            [1u8, 2u8, 3u8, 4u8, 5u8]
        );
    }

    #[test]
    fn test_winning_line_col() {
        let mut board = BingoBoard::new();
        for i in 0..=4 {
            board.board[i][0] = (i + 1) as u8;
        }
        println!("board: {:?}", board.board);
        let called_nums = vec![1, 2, 3, 4, 5];
        assert_eq!(
            board.winning_line(&called_nums).unwrap().0,
            [1u8, 2u8, 3u8, 4u8, 5u8]
        );
    }

    #[test]
    fn test_winning_line_diag() {
        let mut board = BingoBoard::new();
        for i in 0..=4 {
            board.board[i][i] = (i + 1) as u8;
        }
        println!("board: {:?}", board.board);
        let called_nums = vec![1, 2, 3, 4, 5];
        assert_eq!(
            board.winning_line(&called_nums).unwrap().0,
            [1u8, 2u8, 3u8, 4u8, 5u8]
        );
    }

    #[test]
    fn test_winning_line_left_diag() {
        let mut board = BingoBoard::new();
        for i in 0..=4 {
            board.board[4 - i][i] = (i + 1) as u8;
        }
        println!("board: {:?}", board.board);
        let called_nums = vec![1, 2, 3, 4, 5];
        assert_eq!(
            board.winning_line(&called_nums).unwrap().0,
            [5u8, 4u8, 3u8, 2u8, 1u8]
        );
    }

    #[test]
    fn test_parse_game_info() {
        let s: String = String::from(
            "1,2,3,4,5,6,7,8,9,10\n\n8 3 12 47 65\n 6 8 5 3 2\n9 15 48 75 42\n32 14 56 8 3\n
            87 96 52 11 1\n\n8 3 12 47 65\n 6 8 5 3 2\n9 15 48 75 42\n32 14 56 8 3\n87 96 52 11 1\n"
        );
        let (instructions, boards) = parse_game_info(s);
        assert_eq!(instructions, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let b: [[u8; 5]; 5] = [
            [8, 3, 12, 47, 65],
            [6, 8, 5, 3, 2],
            [9, 15, 48, 75, 42],
            [32, 14, 56, 8, 3],
            [87, 96, 52, 11, 1],
        ];
        assert_eq!(boards[0].board, b);
        assert_eq!(boards[1].board, b);
    }
}
