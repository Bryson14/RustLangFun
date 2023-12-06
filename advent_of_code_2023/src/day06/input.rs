/// Time:      7  15   30
/// Distance:  9  40  200
/// turns into
/// [Race{7, 9}, Race{15, 40}, Race{30, 200}]
pub fn read_races(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<u64>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<u64> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            parts
        })
        .collect();

    assert!(lines.len() == 2, "Expected 2 lines, got {}", lines.len());

    // zip the two vecs together
    let times = lines[0].clone();
    let distances = lines[1].clone();

    assert!(
        times.len() == distances.len(),
        "Expected same number of times and distances"
    );

    times
        .iter()
        .enumerate()
        .map(|(i, item)| Race {
            duration: *item,
            record_distance: distances[i],
        })
        .collect()
}

/// Time:      7  15   30
/// Distance:  9  40  200
/// turns into
/// Race{71530, 940200}
pub fn read_race_part_2(input: &str) -> Race {
    let lines: Vec<String> = input.lines().map(|line| {
        line.chars().filter(|c| c.is_ascii_digit()).collect()
    }).collect();

    assert!(lines.len() == 2, "Expected 2 lines, got {}", lines.len());

    let combined_duration: String = lines[0].chars().collect();
    let combined_distance: String = lines[1].chars().collect();

    Race {
        duration: combined_duration.parse().unwrap(),
        record_distance: combined_distance.parse().unwrap(),
    }
}

#[derive(Debug, PartialEq)]
pub struct Race {
    pub duration: u64,
    pub record_distance: u64,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_read_races() {
        let input = String::from(
            "
        Time:      7  15   30
        Distance:  9  40  200
        ",
        );
        let expected = vec![
            Race {
                duration: 7,
                record_distance: 9,
            },
            Race {
                duration: 15,
                record_distance: 40,
            },
            Race {
                duration: 30,
                record_distance: 200,
            },
        ];

        assert_eq!(read_races(&input), expected);
    }

    #[test]
    fn test_read_races_2() {
        let input = String::from(
            "
        Time:      1  101   3000
        Distance:  2  102  987654
        ",
        );
        let expected = vec![
            Race {
                duration: 1,
                record_distance: 2,
            },
            Race {
                duration: 101,
                record_distance: 102,
            },
            Race {
                duration: 3000,
                record_distance: 987654,
            },
        ];

        assert_eq!(read_races(&input), expected);
    }
}
