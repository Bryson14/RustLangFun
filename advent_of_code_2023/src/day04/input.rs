pub fn read_card(input: &str) -> (Vec<u32>, Vec<u32>) {
    // split off the first part of the string before the ':'
    // then split that into a vector of strings, split by '|'
    // then split each of those strings into a vector of strings, split by ' '
    // then parse each of those strings into a vector of u32
    let numbers: Vec<&str> = input
        .split(':')
        .nth(1)
        .unwrap()
        .split('|')
        .collect::<Vec<&str>>();

    let winning_numbers = numbers[0]
        .trim()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let my_numbers = numbers[1]
        .trim()
        .split(' ')
        .filter(|&x| !x.is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    (winning_numbers, my_numbers)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (winning_numbers, my_numbers) = read_card(input);
        assert_eq!(winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(my_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }
}
