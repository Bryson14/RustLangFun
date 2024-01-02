use crate::day11::SpaceImage;
use crate::day11::part1::{expand_space_image, sum_smallest_pairs};

/// # --- Part Two ---
/// The galaxies are much older (and thus much farther apart) than the researcher initially estimated.
/// 
/// Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.
/// 
/// (In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)
/// 
/// Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
pub fn solve(input: &str) {
    let mut space_image = SpaceImage::from_string(input);
    space_image = expand_space_image(&space_image, 1_000_000);
    let sum = sum_smallest_pairs(&space_image);

    println!("The sum of the smallest distances between each pair of galaxies is {}", sum);
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
        let expanded_space_image = expand_space_image(&space_image, 10);
        let ans = sum_smallest_pairs(&expanded_space_image);
        assert_eq!(ans, 1030);

        let expanded_space_image = expand_space_image(&space_image, 100);
        let ans = sum_smallest_pairs(&expanded_space_image);
        assert_eq!(ans, 8410);
    }
}