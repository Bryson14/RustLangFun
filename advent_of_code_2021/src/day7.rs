use crate::read_from_data_dir;

/// # --- Day 7: The Treachery of Whales ---
/// A giant whale has decided your submarine is its next meal, and it's much faster than you are. There's nowhere to run!
///
/// Suddenly, a swarm of crabs (each in its own tiny submarine - it's too deep for them otherwise) zooms in to rescue you! They seem to be preparing to blast a hole in the ocean floor; sensors indicate a massive underground cave system just beyond where they're aiming!
///
/// The crab submarines all need to be aligned before they'll have enough power to blast a large enough hole for your submarine to get through. However, it doesn't look like they'll be aligned before the whale catches you! Maybe you can help?
///
/// There's one major catch - crab submarines can only move horizontally.
///
/// You quickly make a list of the horizontal position of each crab (your puzzle input). Crab submarines have limited fuel, so you need to find a way to make all of their horizontal positions match while requiring them to spend as little fuel as possible.
///
/// For example, consider the following horizontal positions:
///
/// 16,1,2,0,4,2,7,1,2,14
/// This means there's a crab with horizontal position 16, a crab with horizontal position 1, and so on.
///
/// Each change of 1 step in horizontal position of a single crab costs 1 fuel. You could choose any horizontal position to align them all on, but the one that costs the least fuel is horizontal position 2:
/// ```text
/// Move from 16 to 2: 14 fuel
/// Move from 1 to 2: 1 fuel
/// Move from 2 to 2: 0 fuel
/// Move from 0 to 2: 2 fuel
/// Move from 4 to 2: 2 fuel
/// Move from 2 to 2: 0 fuel
/// Move from 7 to 2: 5 fuel
/// Move from 1 to 2: 1 fuel
/// Move from 2 to 2: 0 fuel
/// Move from 14 to 2: 12 fuel
/// ```
/// This costs a total of 37 fuel. This is the cheapest possible outcome; more expensive outcomes include aligning at position 1 (41 fuel), position 3 (39 fuel), or position 10 (71 fuel).
///
/// Determine the horizontal position that the crabs can align to using the least fuel possible. How much fuel must they spend to align to that position?
pub fn part1() {
    let data = read_from_data_dir("day7.txt").unwrap();
    let crab_locations: Vec<i32> = data
        .trim()
        .split(',')
        .map(|s| s.parse().expect("errer parsing"))
        .collect();
    let (fuel, best_position) = least_fuel_location(crab_locations, &movement_cost_linear);
    println!("Day7:1 Fuel used: {} at position: {}", fuel, best_position);
}

/// takes the current positions of the crabs and a movemnet function,
/// returns the total cost of fuel and the best position of the crabs
fn least_fuel_location(crabs: Vec<i32>, movement_cost: &dyn Fn(i32) -> i32) -> (i32, i32) {
    let min_pos: i32 = *crabs.iter().min().unwrap();
    let max_pos: i32 = *crabs.iter().max().unwrap();
    let mut fuel_costs: Vec<i32> = vec![0; (max_pos - min_pos + 1) as usize];

    for pos in min_pos..=max_pos {
        for crab in crabs.iter() {
            fuel_costs[pos as usize] += movement_cost((pos - crab).abs());
        }
    }

    let total_fuel = *fuel_costs.iter().min().unwrap();
    let best_position = fuel_costs.iter().position(|&f| f == total_fuel).unwrap();

    (total_fuel, best_position as i32)
}

/// this treats all movement equally. Every step takes one unit of fuel
fn movement_cost_linear(distance: i32) -> i32 {
    distance
}

/// this treats longer movements differently. Every step of distance takes one more unit of fuel.
/// i.e.
/// ```text
/// 1 -> 1
/// 2 -> 3
/// 3 -> 6
/// ...
/// ```
/// A closed form equation has sped up the function and replace the iterative form
/// cost = (x^2 + x) / 2
fn movement_cost_increment_one(distance: i32) -> i32 {
    // slow, iterative form
    // let mut cost = 0;
    // for step in 0..=distance {
    //     cost += step;
    // }
    // cost

    // closed form equation
    (distance * distance + distance) / 2
}

/// # --- Part Two ---
/// The crabs don't seem interested in your proposed solution. Perhaps you misunderstand crab engineering?
///
/// As it turns out, crab submarine engines don't burn fuel at a constant rate. Instead, each change of 1 step in horizontal position costs 1 more unit of fuel than the last: the first step costs 1, the second step costs 2, the third step costs 3, and so on.
///
/// As each crab moves, moving further becomes more expensive. This changes the best horizontal position to align them all on; in the example above, this becomes 5:
///
/// Move from 16 to 5: 66 fuel
/// Move from 1 to 5: 10 fuel
/// Move from 2 to 5: 6 fuel
/// Move from 0 to 5: 15 fuel
/// Move from 4 to 5: 1 fuel
/// Move from 2 to 5: 6 fuel
/// Move from 7 to 5: 3 fuel
/// Move from 1 to 5: 10 fuel
/// Move from 2 to 5: 6 fuel
/// Move from 14 to 5: 45 fuel
/// This costs a total of 168 fuel. This is the new cheapest possible outcome; the old alignment position (2) now costs 206 fuel instead.
///
/// Determine the horizontal position that the crabs can align to using the least fuel possible so they can make you an escape route! How much fuel must they spend to align to that position?
pub fn part2() {
    let data = read_from_data_dir("day7.txt").unwrap();
    let crab_locations: Vec<i32> = data
        .trim()
        .split(',')
        .map(|s| s.parse().expect("errer parsing"))
        .collect();
    let (fuel, best_position) = least_fuel_location(crab_locations, &movement_cost_increment_one);
    println!("Day7:2 Fuel used: {} at position: {}", fuel, best_position);
}

pub fn is_complete() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_fuel_location() {
        let locations = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            least_fuel_location(locations, &movement_cost_linear),
            (37, 2)
        );
    }

    #[test]
    fn test_least_fuel_location_non_linear() {
        let locations = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(
            least_fuel_location(locations, &movement_cost_increment_one),
            (168, 5)
        );
    }
}
