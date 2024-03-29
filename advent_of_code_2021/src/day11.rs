use crate::read_from_data_dir;

/// # --- Day 11: Dumbo Octopus ---
/// You enter a large cavern full of rare bioluminescent dumbo octopuses! They seem to not like the Christmas lights on your submarine, so you turn them off for now.
///
/// There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly gains energy over time and flashes brightly for a moment when its energy is full. Although your lights are off, maybe you could navigate through the cave without disturbing the octopuses if you could predict when the flashes of light will happen.
///
/// Each octopus has an energy level - your submarine can remotely measure the energy level of each octopus (your puzzle input). For example:
/// ```text
/// 5483143223
/// 2745854711
/// 5264556173
/// 6141336146
/// 6357385478
/// 4167524645
/// 2176841721
/// 6882881134
/// 4846848554
/// 5283751526
/// ```
/// The energy level of each octopus is a value between 0 and 9. Here, the top-left octopus has an energy level of 5, the bottom-right one has an energy level of 6, and so on.
///
/// You can model the energy levels and flashes of light in steps. During a single step, the following occurs:
///
/// First, the energy level of each octopus increases by 1.
/// Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an octopus to have an energy level greater than 9, it also flashes. This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
/// Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
/// Adjacent flashes can cause an octopus to flash on a step even if it begins that step with very little energy. Consider the middle octopus with 1 energy in this situation:
///
/// Before any steps:
/// ```text
/// 11111
/// 19991
/// 19191
/// 19991
/// 11111
/// ```
///
/// After step 1:
/// ```text
/// 34543
/// 40004
/// 50005
/// 40004
/// 34543
/// ```
/// After step 2:
/// ```text
/// 45654
/// 51115
/// 61116
/// 51115
/// 45654
/// ```
/// An octopus is highlighted when it flashed during the given step.
///
/// Here is how the larger example above progresses:
///
/// Before any steps:
/// ```text
/// 5483143223
/// 2745854711
/// 5264556173
/// 6141336146
/// 6357385478
/// 4167524645
/// 2176841721
/// 6882881134
/// 4846848554
/// 5283751526
/// ```
/// After step 1:
/// ```text
/// 6594254334
/// 3856965822
/// 6375667284
/// 7252447257
/// 7468496589
/// 5278635756
/// 3287952832
/// 7993992245
/// 5957959665
/// 6394862637
/// ```
/// After step 2:
/// ```text
/// 8807476555
/// 5089087054
/// 8597889608
/// 8485769600
/// 8700908800
/// 6600088989
/// 6800005943
/// 0000007456
/// 9000000876
/// 8700006848
/// ```
/// After step 3:
/// ```text
/// 0050900866
/// 8500800575
/// 9900000039
/// 9700000041
/// 9935080063
/// 7712300000
/// 7911250009
/// 2211130000
/// 0421125000
/// 0021119000
/// ```
/// After step 4:
/// ```text
/// 2263031977
/// 0923031697
/// 0032221150
/// 0041111163
/// 0076191174
/// 0053411122
/// 0042361120
/// 5532241122
/// 1532247211
/// 1132230211
/// ```
/// After step 5:
/// ```text
/// 4484144000
/// 2044144000
/// 2253333493
/// 1152333274
/// 1187303285
/// 1164633233
/// 1153472231
/// 6643352233
/// 2643358322
/// 2243341322
/// ```
/// After step 6:
/// ```text
/// 5595255111
/// 3155255222
/// 3364444605
/// 2263444496
/// 2298414396
/// 2275744344
/// 2264583342
/// 7754463344
/// 3754469433
/// 3354452433
/// ```
/// After step 7:
/// ```text
/// 6707366222
/// 4377366333
/// 4475555827
/// 3496655709
/// 3500625609
/// 3509955566
/// 3486694453
/// 8865585555
/// 4865580644
/// 4465574644
/// ```
/// After step 8:
/// ```text
/// 7818477333
/// 5488477444
/// 5697666949
/// 4608766830
/// 4734946730
/// 4740097688
/// 6900007564
/// 0000009666
/// 8000004755
/// 6800007755
/// ```
/// After step 9:
/// ```text
/// 9060000644
/// 7800000976
/// 6900000080
/// 5840000082
/// 5858000093
/// 6962400000
/// 8021250009
/// 2221130009
/// 9111128097
/// 7911119976
/// ```
/// After step 10:
/// ```text
/// 0481112976
/// 0031112009
/// 0041112504
/// 0081111406
/// 0099111306
/// 0093511233
/// 0442361130
/// 5532252350
/// 0532250600
/// 0032240000
/// ```
/// After step 10, there have been a total of 204 flashes. Fast forwarding, here is the same configuration every 10 steps:
///
/// After step 20:
/// ```text
/// 3936556452
/// 5686556806
/// 4496555690
/// 4448655580
/// 4456865570
/// 5680086577
/// 7000009896
/// 0000000344
/// 6000000364
/// 4600009543
/// ```
/// After step 30:
/// ```text
/// 0643334118
/// 4253334611
/// 3374333458
/// 2225333337
/// 2229333338
/// 2276733333
/// 2754574565
/// 5544458511
/// 9444447111
/// 7944446119
/// ```
/// After step 40:
/// ```text
/// 6211111981
/// 0421111119
/// 0042111115
/// 0003111115
/// 0003111116
/// 0065611111
/// 0532351111
/// 3322234597
/// 2222222976
/// 2222222762
/// ```
/// After step 50:
/// ```text
/// 9655556447
/// 4865556805
/// 4486555690
/// 4458655580
/// 4574865570
/// 5700086566
/// 6000009887
/// 8000000533
/// 6800000633
/// 5680000538
/// ```
/// After step 60:
/// ```text
/// 2533334200
/// 2743334640
/// 2264333458
/// 2225333337
/// 2225333338
/// 2287833333
/// 3854573455
/// 1854458611
/// 1175447111
/// 1115446111
/// ```
/// After step 70:
/// ```text
/// 8211111164
/// 0421111166
/// 0042111114
/// 0004211115
/// 0000211116
/// 0065611111
/// 0532351111
/// 7322235117
/// 5722223475
/// 4572222754
/// ```
/// After step 80:
/// ```text
/// 1755555697
/// 5965555609
/// 4486555680
/// 4458655580
/// 4570865570
/// 5700086566
/// 7000008666
/// 0000000990
/// 0000000800
/// 0000000000
/// ```
/// After step 90:
/// ```text
/// 7433333522
/// 2643333522
/// 2264333458
/// 2226433337
/// 2222433338
/// 2287833333
/// 2854573333
/// 4854458333
/// 3387779333
/// 3333333333
/// ````
/// After step 100:
/// ```text
/// 0397666866
/// 0749766918
/// 0053976933
/// 0004297822
/// 0004229892
/// 0053222877
/// 0532222966
/// 9322228966
/// 7922286866
/// 6789998766
/// ```
/// After 100 steps, there have been a total of 1656 flashes.
///
/// Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps. How many total flashes are there after 100 steps?
pub fn part1() {
    let data = read_from_data_dir("day11.txt").unwrap();
    let mut octo_grid = OctopusGrid {
        grid: string_to_map(data),
        flashes: 0,
    };
    let steps = 100;

    for _step in 0..steps {
        octo_grid.simulate_time_step();
    }

    println!(
        "Day11:1. After 100 Steps, there have been {} flashes.",
        octo_grid.flashes
    );
}

/// holds the grid and flashes
struct OctopusGrid {
    grid: Vec<Vec<i32>>,
    flashes: i32,
}

impl OctopusGrid {
    /// increments everything in the grid by one
    fn increment_all(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|num| {
                *num += 1;
            })
        });
    }

    /// moves the grid forward one time step
    fn simulate_time_step(&mut self) {
        self.increment_all();
        self.count_flashes();
        self.reset_flashed();
    }

    /// checks if any of the octopuses are over 9, the energy threshold
    fn count_flashes(&mut self) {
        let energy_threshold = 9;
        let mut not_finished = true;

        while not_finished {
            // iterates the entire grid and flashes the octopus if over a level,
            // sets the octopus to -1, and increments its neighbors
            for row in 0..self.grid.len() {
                for col in 0..self.grid[row].len() {
                    if self.grid[row][col] > energy_threshold {
                        self.grid[row][col] = -1;
                        self.flashes += 1;
                        self.increment_adjacent(row, col);
                    }
                }
            }
            // checks to see if any octopuses still over the threshold that haven't been flashed
            not_finished = self
                .grid
                .iter()
                .any(|row| row.iter().any(|&val| val > energy_threshold));
        }
    }

    // resets all flashed octopuses for the time step back to energy level 0
    fn reset_flashed(&mut self) {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut()
                .filter(|&&mut num| num == -1)
                .for_each(|num| {
                    *num = 0;
                })
        });
    }

    /// Increments all the surrounding octopuses of the flashing octopuses by one
    /// The flashed octopus is set to -1
    /// Any octopuses at level -1 are not changed
    fn increment_adjacent(&mut self, row: usize, col: usize) {
        let mut positions: Vec<(usize, usize)> = Vec::new();

        // top left
        if row > 0 && col > 0 {
            positions.push((row - 1, col - 1))
        };

        // top
        if row > 0 {
            positions.push((row - 1, col))
        };

        // top right
        if row > 0 && col < self.grid.len() - 1 {
            positions.push((row - 1, col + 1))
        };

        // left
        if col > 0 {
            positions.push((row, col - 1))
        };

        // right
        if col < self.grid[row].len() - 1 {
            positions.push((row, col + 1))
        };

        // bottom left
        if row < self.grid.len() - 1 && col > 0 {
            positions.push((row + 1, col - 1))
        };

        // bottom
        if row < self.grid.len() - 1 {
            positions.push((row + 1, col))
        };

        // bottom right
        if row < self.grid.len() - 1 && col < self.grid[row].len() - 1 {
            positions.push((row + 1, col + 1))
        };

        for (row, col) in positions {
            if self.grid[row][col] != -1 {
                self.grid[row][col] += 1;
            }
        }
    }

    /// returns true if all the grid is at energy level 0. Meaning they all just flashed.
    fn all_just_flashed(&self) -> bool {
        self.grid.iter().all(|row| row.iter().all(|&val| val == 0))
    }
}

fn string_to_map(data: String) -> Vec<Vec<i32>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("bad parsing") as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

/// # --- Part Two ---
/// It seems like the individual flashes aren't bright enough to navigate. However, you might have a better option: the flashes seem to be synchronizing!
///
/// In the example above, the first time all octopuses flash simultaneously is step 195:
///
/// After step 193:
/// 5877777777
/// 8877777777
/// 7777777777
/// 7777777777
/// 7777777777
/// 7777777777
/// 7777777777
/// 7777777777
/// 7777777777
/// 7777777777
///
/// After step 194:
/// 6988888888
/// 9988888888
/// 8888888888
/// 8888888888
/// 8888888888
/// 8888888888
/// 8888888888
/// 8888888888
/// 8888888888
/// 8888888888
///
/// After step 195:
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// 0000000000
/// If you can calculate the exact moments when the octopuses will all flash simultaneously, you should be able to navigate through the cavern. What is the first step during which all octopuses flash?
pub fn part2() {
    let data = read_from_data_dir("day11.txt").unwrap();
    let mut octo_grid = OctopusGrid {
        grid: string_to_map(data),
        flashes: 0,
    };
    let mut steps = 0;

    while !octo_grid.all_just_flashed() {
        octo_grid.simulate_time_step();
        steps += 1;
    }

    println!(
        "Day11:2. After {} steps, all the octopuses just flashed at once!",
        steps
    );
}

pub fn is_complete() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_map() {
        assert_eq!(
            string_to_map(String::from("123\n456\n789")),
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
        );
    }

    #[test]
    fn test_increment_all() {
        let starting_grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let ending_grid = vec![vec![2, 3, 4], vec![5, 6, 7], vec![8, 9, 10]];
        let mut octo_grid = OctopusGrid {
            grid: starting_grid,
            flashes: 0,
        };
        octo_grid.increment_all();

        assert_eq!(octo_grid.grid, ending_grid);
    }

    #[test]
    fn test_one_step() {
        let starting_grid = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ];

        let end_grid = vec![
            vec![3, 4, 5, 4, 3],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ];
        let mut octo_grid = OctopusGrid {
            grid: starting_grid,
            flashes: 0,
        };
        octo_grid.simulate_time_step();

        assert_eq!(octo_grid.grid, end_grid);
        assert_eq!(octo_grid.flashes, 9);
    }

    #[test]
    fn test_one_step_1() {
        let starting_grid = vec![
            vec![3, 4, 5, 4, 3],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ];

        let end_grid = vec![
            vec![4, 5, 6, 5, 4],
            vec![5, 1, 1, 1, 5],
            vec![6, 1, 1, 1, 6],
            vec![5, 1, 1, 1, 5],
            vec![4, 5, 6, 5, 4],
        ];
        let mut octo_grid = OctopusGrid {
            grid: starting_grid,
            flashes: 0,
        };
        octo_grid.simulate_time_step();

        assert_eq!(octo_grid.grid, end_grid);
        assert_eq!(octo_grid.flashes, 0);
    }

    #[test]
    fn test_reset_grid() {
        let starting_grid = vec![
            vec![3, 4, 5, 4, 3],
            vec![4, -1, -1, -1, 4],
            vec![5, -1, -1, -1, 5],
            vec![4, -1, -1, -1, 4],
            vec![3, 4, 5, 4, 3],
        ];

        let ending_grid = vec![
            vec![3, 4, 5, 4, 3],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ];

        let mut octo_grid = OctopusGrid {
            grid: starting_grid,
            flashes: 0,
        };
        octo_grid.reset_flashed();

        assert_eq!(octo_grid.grid, ending_grid);
    }

    #[test]
    fn test_increment_adjacent_1() {
        let starting_grid = vec![vec![1, 1], vec![1, 1]];

        let ending_grid = vec![vec![1, 2], vec![2, 2]];

        let mut octo_grid = OctopusGrid {
            grid: starting_grid,
            flashes: 0,
        };
        octo_grid.increment_adjacent(0, 0);

        assert_eq!(octo_grid.grid, ending_grid);
    }

    #[test]
    fn test_increment_adjacent_2() {
        let starting_grid = vec![vec![1, 1], vec![1, 1]];

        let ending_grid = vec![vec![2, 2], vec![2, 1]];

        let mut octo_grid = OctopusGrid {
            grid: starting_grid,
            flashes: 0,
        };
        octo_grid.increment_adjacent(1, 1);

        assert_eq!(octo_grid.grid, ending_grid);
    }
}
