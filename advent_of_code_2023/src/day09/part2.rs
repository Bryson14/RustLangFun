use crate::day09::create_diff_map;

use super::read_all_history;

/// # --- Part Two ---
/// Of course, it would be nice to have even more history included in your report. Surely it's safe to just extrapolate backwards as well, right?
///
/// For each history, repeat the process of finding differences until the sequence of differences is entirely zero. Then, rather than adding a zero to the end and filling in the next values of each previous sequence, you should instead add a zero to the beginning of your sequence of zeroes, then fill in new first values for each previous sequence.
///
/// In particular, here is what the third example history looks like when extrapolating back in time:
/// ```
/// 5  10  13  16  21  30  45
///   5   3   3   5   9  15
///    -2   0   2   4   6
///       2   2   2   2
///         0   0   0
/// ```
/// Adding the new values on the left side of each sequence from bottom to top eventually reveals the new left-most history value: 5.
///
/// Doing this for the remaining example data above results in previous values of -3 for the first history and 0 for the second history. Adding all three new values together produces 2.
///
/// Analyze your OASIS report again, this time extrapolating the previous value for each history. What is the sum of these extrapolated values?
pub fn solve(input: &str) {
    let sensor_history = read_all_history(input);
    let mut first_forcasted_values = 0;
    for history in sensor_history.iter() {
        let mut diff_map = create_diff_map(history);
        extrapolate_first_diff_map(&mut diff_map);
        let forecasted_value = diff_map[0].first().unwrap();
        first_forcasted_values += forecasted_value;
    }

    println!("Sum of first forecasted values: {}", first_forcasted_values);
}

// starting at the last row with all zeros, add another zero,
// then for every row, add the value to the left and the value below it
fn extrapolate_first_diff_map(diff_map: &mut Vec<Vec<isize>>) {
    for row_idx in (0..diff_map.len()).rev() {
        if row_idx == diff_map.len() - 1 {
            diff_map[row_idx].insert(0, 0);
        } else {
            // get the last items in the row and the row below it
            let first_item = diff_map[row_idx].first().unwrap().clone();
            let below_item = diff_map[row_idx + 1].first().unwrap().clone();
            diff_map[row_idx].insert(0, first_item - below_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_extrapolate_first_diff_map() {
        // 5  10  13  16  21  30  45
        //   5   3   3   5   9  15
        //   -2   0   2   4   6
        //      2   2   2   2
        //        0   0   0
        let mut diff_map = vec![
            vec![10, 13, 16, 21, 30, 45],
            vec![3, 3, 5, 9, 15],
            vec![0, 2, 4, 6],
            vec![2, 2, 2],
            vec![0, 0],
        ];
        let expected = vec![
            vec![5, 10, 13, 16, 21, 30, 45],
            vec![5, 3, 3, 5, 9, 15],
            vec![-2, 0, 2, 4, 6],
            vec![2, 2, 2, 2],
            vec![0, 0, 0],
        ];
        super::extrapolate_first_diff_map(&mut diff_map);
        assert_eq!(diff_map, expected);
    }
}
