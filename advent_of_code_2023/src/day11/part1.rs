use std::collections::HashMap;

use super::{SpaceImage, Position, calculate_distance};

/// # --- Day 11: Cosmic Expansion ---
/// You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.
/// 
/// He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.
/// 
/// Maybe you can help him with the analysis to speed things up?
/// 
/// The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:
/// ```
/// ...#......
/// .......#..
/// #.........
/// ..........
/// ......#...
/// .#........
/// .........#
/// ..........
/// .......#..
/// #...#.....
/// ```
/// The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.
/// 
/// Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.
/// 
/// In the above example, three columns and two rows contain no galaxies:
/// ```
///    v  v  v
///  ...#......
///  .......#..
///  #.........
/// >..........<
///  ......#...
///  .#........
///  .........#
/// >..........<
///  .......#..
///  #...#.....
///    ^  ^  ^
/// ```
/// These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:
/// ```
/// ....#........
/// .........#...
/// #............
/// .............
/// .............
/// ........#....
/// .#...........
/// ............#
/// .............
/// .............
/// .........#...
/// #....#.......
/// ```
/// Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:
/// ```
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// ............6
/// .............
/// .............
/// .........7...
/// 8....9.......
/// ```
/// In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)
/// 
/// For example, here is one of the shortest paths between galaxies 5 and 9:
/// ```
/// ....1........
/// .........2...
/// 3............
/// .............
/// .............
/// ........4....
/// .5...........
/// .##.........6
/// ..##.........
/// ...##........
/// ....##...7...
/// 8....9.......
/// ```
/// This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:
/// 
/// Between galaxy 1 and galaxy 7: 15
/// Between galaxy 3 and galaxy 6: 17
/// Between galaxy 8 and galaxy 9: 5
/// In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.
/// 
/// Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
pub fn solve(input: &str) {
    let mut space_image = SpaceImage::from_string(input);
    space_image = expand_space_image(&space_image, 2);
    let sum = sum_smallest_pairs(&space_image);

    println!("The sum of the smallest distances between each pair of galaxies is {}", sum);
}

/// expands the space image by adding extra rows or columns where there are no galaxies
/// this will just alter the current positions of the galaxies in the hashmap
/// expansion_ratio is how many times larger the empty rows/columns should be. 
/// So 1 will keep the image the same size, 2 will double the size, 3 will triple the size, etc.
pub fn expand_space_image(image: &SpaceImage, expansion_ratio: usize) -> SpaceImage {
    let mut new_img: HashMap<Position, usize> = HashMap::new();

    let empty_columns = image.get_empty_columns();
    let empty_rows = image.get_empty_rows();

    // iterate through all the positions in the image
    // if the position is less than the empty column, increase its x value by 1. repeat for each column
    // if the position is less than the empty row, increase its y value by 1. repeat for each row
    for (pos, galaxy) in image.img.iter() {
        let new_pos = Position {
            x: pos.x + empty_columns.iter().filter(|&&column| pos.x > column).count() * (expansion_ratio - 1),
            y: pos.y + empty_rows.iter().filter(|&&row| pos.y > row).count() * (expansion_ratio - 1),
        };
        new_img.insert(new_pos, *galaxy);
    }

    let occupied_columns = image.width - empty_columns.len();
    let occupied_rows = image.height - empty_rows.len();

    SpaceImage {
        img: new_img,
        width: occupied_columns + empty_columns.len() * expansion_ratio,
        height: occupied_rows + empty_rows.len() * expansion_ratio,
    }
}

/// calculates the distance from every galaxy to every other galaxy.
pub fn sum_smallest_pairs(image: &SpaceImage) -> usize {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    // iterate through all the galaxies
    for (pos1, galaxy1) in image.img.iter() {
        for (pos2, galaxy2) in image.img.iter() {
            // if the galaxy is the same, skip it
            if pos1 == pos2 {
                continue;
            }

            // so we don't double sum
            if galaxy1 > galaxy2 {
                continue;
            }

            // if we've already calculated the distance between these two galaxies, skip it
            if distances.contains_key(&(*galaxy1, *galaxy2)) {
                continue;
            }

            let distance = calculate_distance(pos1, pos2);

            distances.insert((*galaxy1, *galaxy2), distance);

        }
    }

    distances.iter().map(|(_pair, distance)| distance).sum()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_space_image() {
        let input = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        let space_image = SpaceImage::from_string(input);
        let expanded_space_image = expand_space_image(&space_image, 2);
        assert_eq!(expanded_space_image.width, 13);
        assert_eq!(expanded_space_image.height, 12);
        assert_eq!(expanded_space_image.img.len(), 9);
        assert_eq!(expanded_space_image.img.get(&Position { x: 4, y: 0 }), Some(&0));
        assert_eq!(expanded_space_image.img.get(&Position { x: 9, y: 1 }), Some(&1));
        assert_eq!(expanded_space_image.img.get(&Position { x: 0, y: 2 }), Some(&2));
        assert_eq!(expanded_space_image.img.get(&Position { x: 8, y: 5 }), Some(&3));
        assert_eq!(expanded_space_image.img.get(&Position { x: 1, y: 6 }), Some(&4));
        assert_eq!(expanded_space_image.img.get(&Position { x: 12, y: 7 }), Some(&5));
        assert_eq!(expanded_space_image.img.get(&Position { x: 9, y: 10 }), Some(&6));
        assert_eq!(expanded_space_image.img.get(&Position { x: 0, y: 11 }), Some(&7));
        assert_eq!(expanded_space_image.img.get(&Position { x: 5, y: 11 }), Some(&8));
    }

    #[test]
    fn test_expand_space_image_2() {
        let input = "#.#";

        let space_image = SpaceImage::from_string(input);
        let expanded_space_image = expand_space_image(&space_image, 1);

        assert_eq!(expanded_space_image.width, 3);
        assert_eq!(expanded_space_image.height, 1);
        assert_eq!(expanded_space_image.img.len(), 2);
        let output_img = "#.#\n";
        assert_eq!(expanded_space_image.to_string(), output_img);

        
        let expanded_space_image = expand_space_image(&space_image, 2);
        assert_eq!(expanded_space_image.width, 4);
        assert_eq!(expanded_space_image.height, 1);
        assert_eq!(expanded_space_image.img.len(), 2);
        let output_img = "#..#\n";
        assert_eq!(expanded_space_image.to_string(), output_img);

        let expanded_space_image = expand_space_image(&space_image, 3);
        assert_eq!(expanded_space_image.img.len(), 2);
        let output_img = "#...#\n";
        assert_eq!(expanded_space_image.to_string(), output_img);
        assert_eq!(expanded_space_image.width, 5);
        assert_eq!(expanded_space_image.height, 1);

        let expanded_space_image = expand_space_image(&space_image, 10);
        assert_eq!(expanded_space_image.img.len(), 2);
        let output_img = "#..........#\n";
        assert_eq!(expanded_space_image.to_string(), output_img);
        assert_eq!(expanded_space_image.width, 12);
        assert_eq!(expanded_space_image.height, 1);
    }
}