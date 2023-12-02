pub fn solve(input: &str) {
    // iterate over each line and get the two digits in the string
    let ans = input
        .lines()
        .map(|line| get_numbers_from_line(line))
        .sum::<u32>();

    println!("Answer: {}", ans)
}

fn get_numbers_from_line(line: &str) -> u32 {
    // find all the numbers in the input
    // get only the first NumberPosition and last by position
    // sum the numbers
    // print the sum
    let digits = find_digits(line);
    let spelled_numbers = find_spelled_numbers(line);
    let mut numbers: Vec<NumberPosition> = digits;
    numbers.extend(spelled_numbers);
    numbers.sort_by_key(|np| np.position);

    // get the first and last number
    let first = numbers
        .first()
        .expect("there should be at least one number");
    let last = numbers.last().expect("there should be at least one number");
    let answer = first.number * 10 + last.number;

    println!("{} -> {:?} + {:?} = {}", line, first, last, answer);

    // sum the numbers
    answer
}

/// Gets the position of the first character of spelled numbers in the input string
/// ## Examples
/// - "onediaosd" -> NumberPosition { number: 1, position: 0 }
/// - "aidotwo" -> NumberPosition { number: 2, position: 4 }
fn find_spelled_numbers(input: &str) -> Vec<NumberPosition> {
    let spelled_numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut positions: Vec<NumberPosition> = Vec::new();
    for spelled_number in spelled_numbers.clone() {
        let mut start = 0;
        while let Some(position) = input[start..].find(spelled_number) {
            positions.push(NumberPosition {
                number: spelled_numbers
                    .iter()
                    .position(|&n| n == spelled_number)
                    .unwrap() as u32
                    + 1,
                position: start + position,
            });
            start += position + spelled_number.len();
        }
    }

    // sort the positions by position
    positions.sort_by_key(|np| np.position);
    positions
}

/// Gets the position of the digits in the input string
/// ## Examples
/// - "1a2b3c" -> NumberPosition { number: 1, position: 0 }, NumberPosition { number: 2, position: 2 }, NumberPosition { number: 3, position: 4 }
fn find_digits(input: &str) -> Vec<NumberPosition> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_digit(10))
        .map(|(i, c)| NumberPosition {
            number: c.to_digit(10).unwrap(),
            position: i,
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct NumberPosition {
    number: u32,
    position: usize,
}

// wriate a test for find_digits
// write a test for find_spelled_numbers
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_digits() {
        let input = "1a2b3c";
        let expected = vec![
            NumberPosition {
                number: 1,
                position: 0,
            },
            NumberPosition {
                number: 2,
                position: 2,
            },
            NumberPosition {
                number: 3,
                position: 4,
            },
        ];
        let actual = find_digits(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_digits2() {
        let input = "abc";
        let expected: Vec<NumberPosition> = Vec::new();
        let actual = find_digits(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_spelled_numbers() {
        let input = "onediaosd";
        let expected = vec![NumberPosition {
            number: 1,
            position: 0,
        }];
        let actual = find_spelled_numbers(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_spelled_numbers2() {
        let input = "eightwothree";
        let expected = vec![
            NumberPosition {
                number: 8,
                position: 0,
            },
            {
                NumberPosition {
                    number: 2,
                    position: 4,
                }
            },
            {
                NumberPosition {
                    number: 3,
                    position: 7,
                }
            },
        ];
        let actual = find_spelled_numbers(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_find_spelled_numbers3() {
        let input = "eighthree";
        let expected = vec![
            NumberPosition {
                number: 8,
                position: 0,
            },
            {
                NumberPosition {
                    number: 3,
                    position: 4,
                }
            },
        ];
        let actual = find_spelled_numbers(input);
        assert_eq!(expected, actual);
    }
}
