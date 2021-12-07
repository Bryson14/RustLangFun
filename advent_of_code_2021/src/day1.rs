use crate::{read_from_data_dir, string_to_vec_i32};

/// # Day 1 #
/// You're minding your own business on a ship at sea when the overboard alarm goes off!
/// You rush to see if you can help. Apparently, one of the Elves tripped and accidentally
/// sent the sleigh keys flying into the ocean!
///
/// Before you know it, you're inside a submarine the Elves keep ready for situations like this.
/// It's covered in Christmas lights (because of course it is), and it even has an experimental
/// antenna that should be able to track the keys if you can boost its signal strength high enough;
/// there's a little meter that indicates the antenna's signal strength by displaying 0-50 stars.
///
/// Your instincts tell you that in order to save Christmas,
/// you'll need to get all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the
/// Advent calendar; the second puzzle is unlocked when you complete the first.
/// Each puzzle grants one star. Good luck!
///
/// As the submarine drops below the surface of the ocean, it automatically performs a
/// sonar sweep of the nearby sea floor. On a small screen, the sonar sweep report
/// (your puzzle input) appears: each line is a measurement of the sea floor depth as the
/// sweep looks further and further away from the submarine.
///
/// For example, suppose you had the following report:
/// ```text
/// 199
/// 200
/// 208
/// 210
/// 200
/// 207
/// 240
/// 269
/// 260
/// 263
/// ```
/// This report indicates that, scanning outward from the submarine, the sonar sweep found
/// depths of 199, 200, 208, 210, and so on.
///
/// The first order of business is to figure out how quickly the depth increases,
/// just so you know what you're dealing with - you never know if the keys will get
/// carried into deeper water by an ocean current or a fish or something.
///
/// To do this, count the number of times a depth measurement increases
/// from the previous measurement. (There is no measurement before the first measurement.)
///  In the example above, the changes are as follows:
/// ```rust
/// // 199 (N/A - no previous measurement)
/// // 200 (increased)
/// // 208 (increased)
/// // 210 (increased)
/// // 200 (decreased)
/// // 207 (increased)
/// // 240 (increased)
/// // 269 (increased)
/// // 260 (decreased)
/// // 263 (increased)
/// let ans: i32 = sonar_sweep(vec![199,200,208,210,200,207,240,269,260,263]);
/// assert_eq!(7, ans);
/// ```
/// In this example, there are 7 measurements that are larger than the previous measurement.
///
/// How many measurements are larger than the previous measurement?
pub fn part1() {
    let data = read_from_data_dir("day1.txt").unwrap();
    let vec_ints = string_to_vec_i32(data).unwrap();
    let answer = sonar_sweep(vec_ints);
    println!("Day1:1. The answer for day 1, part 1 is {}", answer);
}

/// Compares the number of increases of the previous measurement and the current
/// ## Example ##
/// if the depths were 1,8,0,2,6,
/// ```text
/// 1 > 8 (increase)
/// 8 > 0 (decrease)
/// 0 > 2 (increase)
/// 2 > 6 (increase)
/// ```
/// So for this example, there would be `3` increases
fn sonar_sweep(depths: Vec<i32>) -> i32 {
    let mut count = 0;
    for (pos, _) in depths.iter().enumerate() {
        if pos == 0 {
            continue;
        }
        if depths[pos] > depths[pos - 1] {
            count += 1;
        }
    }
    count
}

/// ## -- Part 2 -- ##
///
/// Considering every single measurement isn't as useful as you expected: there's just too much noise in the data.
///
/// Instead, consider sums of a three-measurement sliding window. Again considering the above example:
/// ```text
/// 199  A      
/// 200  A B    
/// 208  A B C  
/// 210    B C D
/// 200  E   C D
/// 207  E F   D
/// 240  E F G  
/// 269    F G H
/// 260      G H
/// 263        H
/// ```
/// Start by comparing the first and second three-measurement windows. The measurements in the first window are marked A (199, 200, 208); their sum is 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its sum is 618. The sum of measurements in the second window is larger than the sum of the first, so this first comparison increased.
///
/// Your goal now is to count the number of times the sum of measurements in this sliding window increases from the previous sum. So, compare A with B, then compare B with C, then C with D, and so on. Stop when there aren't enough measurements left to create a new three-measurement sum.
///
/// In the above example, the sum of each three-measurement window is as follows:
///
/// ```
/// // A: 607 (N/A - no previous sum)
/// // B: 618 (increased)
/// // C: 618 (no change)
/// // D: 617 (decreased)
/// // E: 647 (increased)
/// // F: 716 (increased)
/// // G: 769 (increased)
/// // H: 792 (increased)
/// let v: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// let ans: i32 = rolling_sonar_sweep(v);
/// assert_eq!(ans, 5);
/// ```
/// In this example, there are 5 sums that are larger than the previous sum.
///
/// Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?
pub fn part2() {
    let data = read_from_data_dir("day1.txt").unwrap();
    let vec_ints = string_to_vec_i32(data).unwrap();
    let answer = rolling_sonar_sweep(vec_ints);
    println!("Day1:2. The answer for day 1, part 2 is {}", answer);
}

/// Compares the number of increases of the previous window and the current window
/// ## Example ##
/// if the depths were 1,8,0,2,6, and the window was 3, the the sum of the windows would be
/// ```text
/// 1+8+0 = 9
/// 8+0+2 = 10
/// 0+2+6 = 8
/// ```
/// So for this example, there would be `1` increase (from 9 to 10)
fn rolling_sonar_sweep(depths: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = 0;
    const WINDOW: usize = 3;
    for (pos, _) in depths.iter().enumerate() {
        if pos + 1 == WINDOW {
            // setting first prev
            prev = depths[pos] + depths[pos - 1] + depths[pos - 2]
        }
        if pos < WINDOW {
            // can't create rolling data for the first few
            continue;
        }
        let curr = depths[pos] + depths[pos - 1] + depths[pos - 2];
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    count
}

#[cfg(test)]
mod test {
    use super::{rolling_sonar_sweep, sonar_sweep};

    #[test]
    fn test_sonar_sweep() {
        let v: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let ans: i32 = sonar_sweep(v);
        assert_eq!(7, ans);
    }

    #[test]
    fn test_rolling_sonar_sweep() {
        let v: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let ans: i32 = rolling_sonar_sweep(v);
        assert_eq!(ans, 5);
    }
}
