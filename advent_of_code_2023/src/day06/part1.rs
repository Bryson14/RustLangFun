use super::input;

/// # --- Day 6: Wait For It ---
/// The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.
///
/// As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.
///
/// You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.
///
/// As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race. To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.
///
/// The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top. Holding down the button charges the boat, and releasing the button allows the boat to move. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.
///
/// For example:
/// ```
/// Time:      7  15   30
/// Distance:  9  40  200
/// ```
/// This document describes three races:
///
/// - The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
/// - The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
/// - The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
/// Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond.
///
/// So, because the first race lasts 7 milliseconds, you only have a few options:
///
/// - Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
/// - Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
/// - Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
/// - Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
/// - Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
/// - Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
/// - Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
/// - Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
///
/// Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.
///
/// In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.
///
/// In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.
///
/// To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).
///
/// Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?
pub fn solve(input: &str) {
    let races = input::read_races(input);

    // calcuate the ways to win for each race, then mutipley
    let answer = races.iter().map(|race| calculate_ways_to_win(race)).product::<u128>();

    println!("Multiplied ways to win: {}", answer);
}

// gets all the holding times that would beat the record
pub fn calculate_ways_to_win(race: &input::Race) -> u128 {
    let mut ways_to_win = 0;
    for hold_time in 0..race.duration {
        let boat_distance = calculate_boat_distance(race.duration, hold_time);
        if boat_distance > race.record_distance {
            ways_to_win += 1;
        }
    }
    ways_to_win
}

// gets the number of holding times that would beat the record
pub fn calculate_wins_by_roots(race: &input::Race) -> u32 {
    let a = -1.0;
    let b = race.duration as f64;
    let c = -1.0 * (race.record_distance as f64);

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real roots
        0.0 as u32
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let mut root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let mut root2 = (-b - sqrt_discriminant) / (2.0 * a);

        // round up and down to get the number of whole numbers between the roots
        root1 = (root1 + 1.0).floor();
        root2 = (root2 - 1.0).ceil();

        // include + 1 because this is inclusive range of possible options
        (root1 - root2).abs() as u32 + 1
    }
}

fn calculate_boat_distance(race_time: u64, hold_time: u64) -> u64 {
    if hold_time >= race_time {
        return 0;
    } else if hold_time == 0 {
        return 0;
    } else {
        return hold_time * (race_time - hold_time);
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_boat_distance() {
        assert_eq!(calculate_boat_distance(7, 0), 0);
        assert_eq!(calculate_boat_distance(7, 1), 6);
        assert_eq!(calculate_boat_distance(7, 2), 10);
        assert_eq!(calculate_boat_distance(7, 3), 12);
        assert_eq!(calculate_boat_distance(7, 4), 12);
        assert_eq!(calculate_boat_distance(7, 5), 10);
        assert_eq!(calculate_boat_distance(7, 6), 6);
        assert_eq!(calculate_boat_distance(7, 7), 0);
    }

    #[test]
    fn test_ways_to_win() {
        let race = input::Race {
            duration: 7,
            record_distance: 9,
        };

        assert_eq!(calculate_ways_to_win(&race), 4);

        let race = input::Race {
            duration: 15,
            record_distance: 40,
        };

        assert_eq!(calculate_ways_to_win(&race), 8);

        let race = input::Race {
            duration: 30,
            record_distance: 200,
        };

        assert_eq!(calculate_ways_to_win(&race), 9);
    }

    #[test]
    fn test_calculate_wins_by_roots() {

        let race = input::Race {
            duration: 7,
            record_distance: 9,
        };

        assert_eq!(calculate_wins_by_roots(&race), 4);

        let race = input::Race {
            duration: 15,
            record_distance: 40,
        };

        assert_eq!(calculate_wins_by_roots(&race), 8);

        let race = input::Race {
            duration: 30,
            record_distance: 200,
        };

        assert_eq!(calculate_wins_by_roots(&race), 9);
    }
}
