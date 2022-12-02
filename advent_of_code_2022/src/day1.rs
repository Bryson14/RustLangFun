use crate::utils::read_data;

const FILE: &str = "day1.txt";
const DAY: &str = "{{ DAY 1 }}";

/// Calorie Counting
/// This list represents the Calories of the food carried by five Elves:
///
/// The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
/// The second Elf is carrying one food item with 4000 Calories.
/// The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
/// The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
/// The fifth Elf is carrying one food item with 10000 Calories.
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).
///
/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
pub fn part1() {
    let data = read_data(FILE);
    let elves = sum_each_elf(data);
    let highest = elves.iter().max().unwrap();
    let idx = elves.iter().find(|c| c == &highest).unwrap();
    println!(
        "{DAY} Max cal is with elf {} with {} calories.",
        idx, highest
    );
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
pub fn part2() {
    let data = read_data(FILE);
    const N: usize = 3;
    let mut elves = sum_each_elf(data);
    elves.sort();
    elves.reverse();
    let top3: u32 = elves.iter().take(N).sum();
    println!("{DAY} The top {N} elves are carrying {top3} calories.")
}

fn sum_each_elf(data: String) -> Vec<u32> {
    let mut elves = Vec::new();
    let mut calories = 0;
    for line in data.lines() {
        match line.parse::<u32>() {
            Ok(num) => {
                calories += num;
            }
            Err(_) => {
                elves.push(calories);
                calories = 0;
            }
        }
    }
    elves
}
