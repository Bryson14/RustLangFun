use std::collections::{HashMap, HashSet};

use super::{find_start_pos, part1::trace_pipe, read_map, Pipe, Position};

/// # --- Part Two ---
/// You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?
///
/// To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:
/// ```
/// ...........
/// .S-------7.
/// .|F-----7|.
/// .||.....||.
/// .||.....||.
/// .|L-7.F-J|.
/// .|..|.|..|.
/// .L--J.L--J.
/// ...........
/// ```
/// The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:
///
/// ```
/// ...........
/// .S-------7.
/// .|F-----7|.
/// .||OOOOO||.
/// .||OOOOO||.
/// .|L-7OF-J|.
/// .|II|O|II|.
/// .L--JOL--J.
/// .....O.....
/// ```
/// In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:
/// ```
/// ..........
/// .S------7.
/// .|F----7|.
/// .||OOOO||.
/// .||OOOO||.
/// .|L-7F-J|.
/// .|II||II|.
/// .L--JL--J.
/// ..........
/// ```
/// In both of the above examples, 4 tiles are enclosed by the loop.
///
/// Here's a larger example:
/// ```
/// .F----7F7F7F7F-7....
/// .|F--7||||||||FJ....
/// .||.FJ||||||||L7....
/// FJL7L7LJLJ||LJ.L-7..
/// L--J.L7...LJS7F-7L7.
/// ....F-J..F7FJ|L7L7L7
/// ....L7.F7||L7|.L7L7|
/// .....|FJLJ|FJ|F7|.LJ
/// ....FJL-7.||.||||...
/// ....L---J.LJ.LJLJ...
/// ```
/// The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):
/// ```
/// OF----7F7F7F7F-7OOOO
/// O|F--7||||||||FJOOOO
/// O||OFJ||||||||L7OOOO
/// FJL7L7LJLJ||LJIL-7OO
/// L--JOL7IIILJS7F-7L7O
/// OOOOF-JIIF7FJ|L7L7L7
/// OOOOL7IF7||L7|IL7L7|
/// OOOOO|FJLJ|FJ|F7|OLJ
/// OOOOFJL-7O||O||||OOO
/// OOOOL---JOLJOLJLJOOO
/// ```
/// In this larger example, 8 tiles are enclosed by the loop.
///
/// Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:
/// ```
/// FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJ7F7FJ-
/// L---JF-JLJ.||-FJLJJ7
/// |F|F-JF---7F7-L7L|7|
/// |FFJF7L7F-JF7|JL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L
/// ```
/// Here are just the tiles that are enclosed by the loop marked with I:
/// FF7FSF7F7F7F7F7F---7
/// L|LJ||||||||||||F--J
/// FL-7LJLJ||||||LJL-77
/// F--JF--7||LJLJIF7FJ-
/// L---JF-JLJIIIIFJLJJ7
/// |F|F-JF---7IIIL7L|7|
/// |FFJF7L7F-JF7IIL---7
/// 7-L-JL7||F7|L7F-7F7|
/// L.L7LFJ|||||FJL7||LJ
/// L7JLJL-JLJLJL--JLJ.L
/// ``````
/// In this last example, 10 tiles are enclosed by the loop.
///
/// Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?
pub fn solve(input: &str) {
    let map = read_map(input);
    // contains all the tiles that are part of the loop
    let dis_map = trace_pipe(&map);
    let mut cleaned_map = map.clone();
    remove_non_loop_tiles(&mut cleaned_map, &dis_map);
    replace_start_tile(&mut cleaned_map, &dis_map);
    let enclosed_tiles = find_enclosed_tiles(&cleaned_map);

    println!("Number of enclosed tiles: {}", enclosed_tiles.len());
}

// turns any pipe that is not in the main loop into ground . tiles
fn remove_non_loop_tiles(map: &mut HashMap<Position, Pipe>, dis_map: &HashMap<Position, usize>) {
    for (pos, pipe) in map.iter_mut() {
        if pipe == &Pipe::Empty {
            continue;
        }
        if !dis_map.contains_key(pos) {
            *pipe = Pipe::Empty;
        }
    }
}

/// Replaces the start tile with the correct pipe
fn replace_start_tile(map: &mut HashMap<Position, Pipe>, dis_map: &HashMap<Position, usize>) {
    let start_pos = find_start_pos(&map).expect("No start position found in map");
    // a hash set with the didfferent pipes
    let mut maybe_s: HashSet<Pipe> = vec![
        Pipe::Vertical,
        Pipe::Horizontal,
        Pipe::ElbowNorthEast,
        Pipe::ElbowNorthWest,
        Pipe::ElbowSouthEast,
        Pipe::ElbowSouthWest,
    ]
    .into_iter()
    .collect();

    // get the two positions that are connected to the start position, i.e. value = 1
    let start_neighbors: Vec<&Position> = dis_map
        .iter()
        .filter(|(_, distance)| **distance == 1)
        .map(|(pos, _)| pos)
        .collect();
    assert!(start_neighbors.len() == 2);

    if start_neighbors.contains(&&Position {
        x: start_pos.x + 1,
        y: start_pos.y,
    }) {
        // right neighbor
        maybe_s.remove(&Pipe::ElbowNorthWest);
        maybe_s.remove(&Pipe::ElbowSouthWest);
        maybe_s.remove(&Pipe::Vertical);
    }
    if start_neighbors.contains(&&Position {
        x: start_pos.x - 1,
        y: start_pos.y,
    }) {
        // left
        maybe_s.remove(&Pipe::ElbowNorthEast);
        maybe_s.remove(&Pipe::ElbowSouthEast);
        maybe_s.remove(&Pipe::Vertical);
    }
    if start_neighbors.contains(&&Position {
        x: start_pos.x,
        y: start_pos.y + 1,
    }) {
        // below
        maybe_s.remove(&Pipe::ElbowNorthEast);
        maybe_s.remove(&Pipe::ElbowNorthWest);
        maybe_s.remove(&Pipe::Horizontal);
    }
    if start_neighbors.contains(&&Position {
        x: start_pos.x,
        y: start_pos.y - 1,
    }) {
        // above
        maybe_s.remove(&Pipe::ElbowSouthEast);
        maybe_s.remove(&Pipe::ElbowSouthWest);
        maybe_s.remove(&Pipe::Horizontal);
    }

    assert!(maybe_s.len() == 1);
    let pipe = maybe_s.iter().next().unwrap().clone();
    map.insert(start_pos, pipe);
}

/// Iterates over the entire map and checks it a pipe is enclosed by the main loop by
/// counting the number of times it crosses the loop. If it cross the main loop an even number of times, it is not enclosed.
/// If it crosses the main loop an odd number of times either vertically or horizontally, it is enclosed.
/// When iterating on top of a parallel pipe, we have to track if the pipe is a turns back or continues perpendicularly
fn find_enclosed_tiles(cleaned_map: &HashMap<Position, Pipe>) -> HashSet<Position> {
    let max_x = cleaned_map.keys().map(|pos| pos.x).max().unwrap();
    let max_y = cleaned_map.keys().map(|pos| pos.y).max().unwrap();
    let mut enclosed_tiles = HashSet::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = Position { x, y };
            if let Some(pipe) = cleaned_map.get(&pos) {
                if pipe != &Pipe::Empty {
                    continue;
                }
                // horizontal scan, checking just to the right of each pipe for odd crossings
                let h_crossings = scan_right(&cleaned_map, &pos, max_x);
                if h_crossings % 2 == 1 {
                    enclosed_tiles.insert(pos);
                }

                // vertical scan, checking just below each pipe for odd crossings
                let v_crossings = scan_down(&cleaned_map, &pos, max_y);
                if v_crossings % 2 == 1 {
                    enclosed_tiles.insert(pos);
                }
            }
        }
    }
    enclosed_tiles
}

// counts the number of crossing to the right from pos
fn scan_right(cleaned_map: &HashMap<Position, Pipe>, pos: &Position, max_x: usize) -> usize {
    let mut x = pos.x + 1;
    let mut crossings = 0;
    // for tracking U-turns and S-turns
    let mut up = None;
    while x <= max_x {
        let pipe = cleaned_map[&Position { x: x, y: pos.y }];
        match pipe {
            Pipe::Vertical => {
                crossings += 1;
            }
            Pipe::Horizontal => {
                assert!(up.is_some());
            }
            Pipe::ElbowNorthEast => {
                assert!(up.is_none());
                up = Some(true);
            }
            Pipe::ElbowNorthWest => {
                assert!(up.is_some());
                if up == Some(false) {
                    crossings += 1;
                }
                up = None;
            }
            Pipe::ElbowSouthEast => {
                assert!(up.is_none());
                up = Some(false);
            }
            Pipe::ElbowSouthWest => {
                assert!(up.is_some());
                if up == Some(true) {
                    crossings += 1;
                }
                up = None;
            }
            Pipe::Start => {
                panic!("Start pipe should have been replaced by now")
            }
            Pipe::Empty => {}
        }
        x += 1;
    }

    crossings
}

fn scan_down(cleaned_map: &HashMap<Position, Pipe>, pos: &Position, max_y: usize) -> usize {
    let mut y = pos.y + 1;
    let mut crossings = 0;
    // for tracking U-turns and S-turns
    let mut left = None;
    while y <= max_y {
        let pipe = cleaned_map[&Position { x: pos.x, y: y }];
        match pipe {
            Pipe::Vertical => {
                assert!(left.is_some());
            }
            Pipe::Horizontal => {
                crossings += 1;
            }
            Pipe::ElbowNorthEast => {
                assert!(left.is_some());
                if left == Some(true) {
                    crossings += 1;
                }
                left = None;
            }
            Pipe::ElbowNorthWest => {
                assert!(left.is_some());
                if left == Some(false) {
                    crossings += 1;
                }
                left = None;
            }
            Pipe::ElbowSouthEast => {
                assert!(left.is_none());
                left = Some(false);
            }
            Pipe::ElbowSouthWest => {
                assert!(left.is_none());
                left = Some(true);
            }
            Pipe::Start => {
                panic!("Start pipe should have been replaced by now")
            }
            Pipe::Empty => {}
        }
        y += 1;
    }

    crossings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_enclosed_tiles() {
        let map = read_map(
            "...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........",
        );
        let mut cleaned_map = map.clone();
        let dis_map = trace_pipe(&map);
        remove_non_loop_tiles(&mut cleaned_map, &dis_map);
        replace_start_tile(&mut cleaned_map, &dis_map);
        let enclosed_tiles = find_enclosed_tiles(&cleaned_map);
        assert_eq!(enclosed_tiles.len(), 4);
        assert!(enclosed_tiles.contains(&Position { x: 2, y: 6 }));
        assert!(enclosed_tiles.contains(&Position { x: 3, y: 6 }));
        assert!(enclosed_tiles.contains(&Position { x: 7, y: 6 }));
        assert!(enclosed_tiles.contains(&Position { x: 8, y: 6 }));
    }

    #[test]
    fn test_remove_non_loop_pipes() {
        let input = r#"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
            "#;

        let expected = r#"
            .....
            .S-7.
            .|.|.
            .L-J.
            .....
                "#;

        let mut map = read_map(input);
        let expected_map = read_map(expected);
        let dis_map = trace_pipe(&map);
        remove_non_loop_tiles(&mut map, &dis_map);
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_replace_start_tile() {
        let input = "
        ...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let map = read_map(input);
        let dis_map = trace_pipe(&map);
        let mut cleaned_map = map.clone();
        remove_non_loop_tiles(&mut cleaned_map, &dis_map);
        let start_pos = find_start_pos(&cleaned_map).expect("No start position found in map");
        assert!(cleaned_map[&start_pos] == Pipe::Start);
        replace_start_tile(&mut cleaned_map, &dis_map);
        assert!(cleaned_map[&start_pos] == Pipe::ElbowSouthEast);
    }
}
