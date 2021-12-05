use crate::{parse_strs_to_ints, read_from_data_dir};

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
/// // 199
/// // 200
/// // 208
/// // 210
/// // 200
/// // 207
/// // 240
/// // 269
/// // 260
/// // 263
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
///
pub fn part1() {
    println!("Here in day1_1");

    let data = read_from_data_dir("day1.txt").unwrap();
    let vec_strings: Vec<String> = data.split(" ").map(|x| String::from(x.trim())).collect();
    let vec_ints = parse_strs_to_ints(vec_strings);
    let answer = sonar_sweep(vec_ints);
    println!("The answer for day 1, part 1 is {}", answer);
}

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

pub fn part2() {
    let data = read_from_data_dir("day1.txt").unwrap();
    let vec_strings: Vec<String> = data.split(" ").map(|x| String::from(x.trim())).collect();
    let vec_ints = parse_strs_to_ints(vec_strings);
    let answer = dive(vec_ints);
    println!("The answer for day 1, part 2 is {}", answer);
}

fn dive(depth: Vec<i32>) -> i32 {
    5 * depth.len() as i32
}

#[cfg(test)]
mod test {
    use super::sonar_sweep;

    #[test]
    fn test_sonar_sweep() {
        let v: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let ans: i32 = sonar_sweep(v);
        assert_eq!(7, ans);
    }
}
