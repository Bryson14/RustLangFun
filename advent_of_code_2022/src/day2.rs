use crate::utils::read_data;

const FILE: &str = "day2.txt";
const DAY: &str = "{{ DAY 2 }}";

const OPP_ROCK: &str = "A";
const OPP_PAPER: &str = "B";
const OPP_SCISSORS: &str = "C";
const ME_ROCK: &str = "X";
const ME_PAPER: &str = "Y";
const ME_SCISSORS: &str = "Z";
const OUTCOME_LOSE: &str = "X";
const OUTCOME_DRAW: &str = "Y";
const OUTCOME_WIN: &str = "Z";
const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;
const LOSE: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

/// Rock Paper Scissors
/// What would your total score be if everything goes exactly according to your strategy guide?
pub fn part1() {
    let data = read_data(FILE);
    let total_score: i32 = data
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|s| get_score(s[0], s[1]))
        .sum();
    println!("{DAY} Total score of strategy 1 is {total_score}");
}

pub fn part2() {
    let data = read_data(FILE);
    let total_score: i32 = data
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|s| get_outcome_score(s[0], s[1]))
        .sum();
    println!("{DAY} Total score of strategy 2 is {total_score}");
}

fn get_score(opp_strat: &str, my_strat: &str) -> i32 {
    match (opp_strat, my_strat) {
        (OPP_ROCK, ME_ROCK) => DRAW + SCORE_ROCK,
        (OPP_PAPER, ME_PAPER) => DRAW + SCORE_PAPER,
        (OPP_SCISSORS, ME_SCISSORS) => DRAW + SCORE_SCISSORS,
        (OPP_SCISSORS, ME_ROCK) => WIN + SCORE_ROCK,
        (OPP_ROCK, ME_PAPER) => WIN + SCORE_PAPER,
        (OPP_PAPER, ME_SCISSORS) => WIN + SCORE_SCISSORS,
        (OPP_PAPER, ME_ROCK) => LOSE + SCORE_ROCK,
        (OPP_SCISSORS, ME_PAPER) => LOSE + SCORE_PAPER,
        (OPP_ROCK, ME_SCISSORS) => LOSE + SCORE_SCISSORS,
        (_, _) => unreachable!(),
    }
}

fn get_outcome_score(opp_strat: &str, outcome: &str) -> i32 {
    match (opp_strat, outcome) {
        (OPP_ROCK, OUTCOME_DRAW) => DRAW + SCORE_ROCK,
        (OPP_PAPER, OUTCOME_DRAW) => DRAW + SCORE_PAPER,
        (OPP_SCISSORS, OUTCOME_DRAW) => DRAW + SCORE_SCISSORS,
        (OPP_SCISSORS, OUTCOME_WIN) => WIN + SCORE_ROCK,
        (OPP_ROCK, OUTCOME_WIN) => WIN + SCORE_PAPER,
        (OPP_PAPER, OUTCOME_WIN) => WIN + SCORE_SCISSORS,
        (OPP_PAPER, OUTCOME_LOSE) => LOSE + SCORE_ROCK,
        (OPP_SCISSORS, OUTCOME_LOSE) => LOSE + SCORE_PAPER,
        (OPP_ROCK, OUTCOME_LOSE) => LOSE + SCORE_SCISSORS,
        (_, _) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_score() {
        assert!(get_score("A", "Y") == 8);
        assert!(get_score("B", "X") == 1);
        assert!(get_score("C", "Z") == 6);
    }

    #[test]
    fn test_get_outcome_score() {
        assert!(get_outcome_score("A", "Y") == 4);
        assert!(get_outcome_score("B", "X") == 1);
        assert!(get_outcome_score("C", "Z") == 7);
    }
}
