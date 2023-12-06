use std::ops::Range;

use super::input;

/// # --- Part Two ---
/// Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.
///
/// The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:
///
/// seeds: 79 14 55 13
/// This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.
///
/// Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.
///
/// In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.
///
/// Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?
pub fn solve(input: &str) {
    let (seeds, charts) = input::read_input(input);

    let new_seeds = convert_seeds_to_ranges(seeds);

    // print seeds len
    println!("Number of seeds: {}", new_seeds.len());

    let answer = new_seeds
        .iter()
        .map(|seed_range| map_seed_range_to_location(seed_range.clone(), &charts))
        .min()
        .unwrap();

    println!("Minimum location of all seed ranges: {}", answer);
}

fn map_seed_range_to_location(seed_range: Range<u64>, charts: &Vec<input::ConversionChart>) -> u64 {
    let mut ranges = vec![seed_range];
    let mut next_ranges = vec![];

    // passes all the ranges through the each chart
    for chart in charts {
        for range in ranges.iter() {
            let converted_ranges = chart.map_range_to_ranges(range.clone());
            next_ranges.extend(converted_ranges);
        }

        ranges = next_ranges;
        next_ranges = vec![];
    }

    // find the minimum value in the ranges
    ranges.iter().map(|range| range.start).min().unwrap()
}

// converts a vector of seeds into a vector of ranges
// each vector should be split into pairs
// 79 14 55 13 -> (79, 14), (55, 13)
// the ranges are then (79, 80, 81 ... 92), (55, 56, 57 ... 67)
fn convert_seeds_to_ranges(seeds: Vec<u64>) -> Vec<Range<u64>> {
    // chunks(2) will split the vector into pairs
    assert_eq!(seeds.len() % 2, 0);
    seeds
        .chunks(2)
        .map(|pair| {
            let start = pair[0];
            let len = pair[1];
            start..start + len
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Instant;

    #[test]
    fn test_convert_seeds_to_ranges() {
        let seeds = vec![79, 14, 55, 13];
        let expected = vec![79..14, 55..13];
        let actual = convert_seeds_to_ranges(seeds);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_main() {
        let input_data = String::from(
            "seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48
                
                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15
                
                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4
                
                water-to-light map:
                88 18 7
                18 25 70
                
                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13
                
                temperature-to-humidity map:
                0 69 1
                1 0 69
                
                humidity-to-location map:
                60 56 37
                56 93 4
                ",
        );

        // run this 100 times and time and average  the run time
        let start = Instant::now();
        for _ in 0..1000 {
            solve(input_data.as_str());
        }
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        let time_execution = duration.as_secs_f64() / 1000.0;
        let time_per_seed = time_execution / (14.0 + 13.0);

        println!("Time per seed: {}", time_per_seed);

        assert!(false);
    }
}
