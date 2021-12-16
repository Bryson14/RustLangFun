use crate::read_from_data_dir;

/// #--- Day 3: Binary Diagnostic ---#
/// The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.
///
/// The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can tell you many useful things about the conditions of the submarine. The first parameter to check is the power consumption.
///
/// You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the gamma rate and the epsilon rate). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
///
/// Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers in the diagnostic report. For example, given the following diagnostic report:
/// ```text
/// 00100
/// 11110
/// 10110
/// 10111
/// 10101
/// 01111
/// 00111
/// 11100
/// 10000
/// 11001
/// 00010
/// 01010
/// ```
/// Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.
///
/// The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.
///
/// The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.
///
/// So, the gamma rate is the binary number 10110, or 22 in decimal.
///
/// The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal. Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.
///
/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)
pub fn part1() {
    let data = read_from_data_dir("day3.txt").unwrap();
    let binary_strings: Vec<&str> = data.lines().collect();
    let (gamma, epsilon) = binary_diagnostic_1(binary_strings);

    println!(
        "Day3:1 gamma = {} and epsilon = {} Multiplied is {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

/// returns gamma and epsilon.
fn binary_diagnostic_1(binary: Vec<&str>) -> (isize, isize) {
    let bit_size = binary[0].len();
    let input_length = binary.len();
    let mut gamma_most_common_votes: Vec<i32> = vec![0; bit_size];
    binary.iter().for_each(|s| {
        for (i, c) in s.chars().enumerate() {
            match c {
                '1' => gamma_most_common_votes[i] += 1,
                '0' => (),
                _ => unreachable!(),
            }
        }
    });

    let mut gamma = String::new();
    let mut epsilon = String::new();

    gamma_most_common_votes.iter().for_each(|&amount| {
        if amount > input_length as i32 / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    });

    let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
    (gamma_int, epsilon_int)
}

/// # --- Part Two ---#
/// Next, you should verify the life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.
///
/// Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in your diagnostic report - finding them is the tricky part. Both values are located using a similar process that involves filtering out values until only one remains. Before searching for either rating value, start with the full list of binary numbers from your diagnostic report and consider just the first bit of those numbers. Then:
///
/// Keep only numbers selected by the bit criteria for the type of rating value for which you are searching. Discard numbers which do not match the bit criteria.
/// If you only have one number left, stop; this is the rating value for which you are searching.
/// Otherwise, repeat the process, considering the next bit to the right.
/// The bit criteria depends on which type of rating value you want to find:
///
/// To find oxygen generator rating, determine the most common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 1 in the position being considered.
/// To find CO2 scrubber rating, determine the least common value (0 or 1) in the current bit position, and keep only numbers with that bit in that position. If 0 and 1 are equally common, keep values with a 0 in the position being considered.
/// For example, to determine the oxygen generator rating value using the same example diagnostic report from above:
///
/// Start with all 12 numbers and consider only the first bit of each number. There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position: 11110, 10110, 10111, 10101, 11100, 10000, and 11001.
/// Then, consider the second bit of the 7 remaining numbers: there are more 0 bits (4) than 1 bits (3), so keep only the 4 numbers with a 0 in the second position: 10110, 10111, 10101, and 10000.
/// In the third position, three of the four numbers have a 1, so keep those three: 10110, 10111, and 10101.
/// In the fourth position, two of the three numbers have a 1, so keep those two: 10110 and 10111.
/// In the fifth position, there are an equal number of 0 bits and 1 bits (one each). So, to find the oxygen generator rating, keep the number with a 1 in that position: 10111.
/// As there is only one number left, stop; the oxygen generator rating is 10111, or 23 in decimal.
/// Then, to determine the CO2 scrubber rating value from the same example above:
///
/// Start again with all 12 numbers and consider only the first bit of each number. There are fewer 0 bits (5) than 1 bits (7), so keep only the 5 numbers with a 0 in the first position: 00100, 01111, 00111, 00010, and 01010.
/// Then, consider the second bit of the 5 remaining numbers: there are fewer 1 bits (2) than 0 bits (3), so keep only the 2 numbers with a 1 in the second position: 01111 and 01010.
/// In the third position, there are an equal number of 0 bits and 1 bits (one each). So, to find the CO2 scrubber rating, keep the number with a 0 in that position: 01010.
/// As there is only one number left, stop; the CO2 scrubber rating is 01010, or 10 in decimal.
/// Finally, to find the life support rating, multiply the oxygen generator rating (23) by the CO2 scrubber rating (10) to get 230.
///
/// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)
pub fn part2() {
    let data = read_from_data_dir("day3.txt").unwrap();
    let binary_strings: Vec<&str> = data.lines().collect();
    let (o2_rating, co2_rating) = oxygen_co2_scrubber_ratings(binary_strings);

    println!(
        "Day3:2. oxygen rating = {} and co2 scrubber rating = {} Multiplied is {}",
        o2_rating,
        co2_rating,
        o2_rating * co2_rating
    );
}

fn oxygen_co2_scrubber_ratings(binary: Vec<&str>) -> (isize, isize) {
    let mut oxygen_rating = String::new();
    let mut co2_scrubber = String::new();
    let mut filtered = binary.clone();

    // finding oxygen rating. None means to choose '1' at ith position
    for i in 0..binary[0].len() {
        match get_most_common(&filtered, i) {
            Some(1) | None => oxygen_rating.push('1'),
            Some(0) => oxygen_rating.push('0'),
            _ => unreachable!(),
        };

        filtered = filtered
            .iter()
            .filter(|&&s| s.starts_with(&oxygen_rating))
            .copied()
            .collect();

        if filtered.len() == 1 {
            oxygen_rating = String::from(filtered[0]);
            break;
        }
    }

    // finding CO2 scrubber rating. None means to choose '0' at ith position
    let mut filtered = binary.clone();
    for i in 0..binary[0].len() {
        match get_most_common(&filtered, i) {
            Some(1) | None => co2_scrubber.push('0'),
            Some(0) => co2_scrubber.push('1'),
            _ => unreachable!(),
        };

        filtered = filtered
            .iter()
            .filter(|&&s| s.starts_with(&co2_scrubber))
            .copied()
            .collect();

        if filtered.len() == 1 {
            co2_scrubber = String::from(filtered[0]);
            break;
        }
    }
    (
        isize::from_str_radix(&oxygen_rating, 2).unwrap(),
        isize::from_str_radix(&co2_scrubber, 2).unwrap(),
    )
}

fn get_most_common(binary: &[&str], idx: usize) -> Option<i32> {
    let ones_count = binary
        .iter()
        .filter(|s| s.chars().nth(idx).unwrap() == '1')
        .count();

    if ones_count == binary.len() / 2 && binary.len() % 2 == 0 {
        None
    } else if ones_count > binary.len() / 2 {
        Some(1)
    } else {
        Some(0)
    }
}

pub fn is_complete() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_diagnostic_1() {
        let data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let (gamma, epsilon) = binary_diagnostic_1(data.lines().collect());
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn test_oxygen_co2_scrubber_ratings() {
        let data =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let (o2, co2) = oxygen_co2_scrubber_ratings(data.lines().collect());
        assert_eq!(o2, 23);
        assert_eq!(co2, 10);
    }

    #[test]
    fn test_get_most_common() {
        let data = vec!["00100", "01111", "00111", "00010", "01010"];
        assert_eq!(get_most_common(&data, 1), Some(0));
        assert_eq!(get_most_common(&data, 0), Some(0));
    }

    #[test]
    fn test_get_most_common_2() {
        let data = vec!["11111", "00000", "11111", "11110"];
        assert_eq!(get_most_common(&data, 1), Some(1));
        assert_eq!(get_most_common(&data, 0), Some(1));
        assert_eq!(get_most_common(&data, 4), None);
    }
}
