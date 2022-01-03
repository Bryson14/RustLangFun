use crate::read_from_data_dir;

/// # --- Day 20: Trench Map ---
/// With the scanners fully deployed, you turn their attention to mapping the floor of the ocean trench.
///
/// When you get back the image from the scanners, it seems to just be random noise. Perhaps you can combine an image enhancement algorithm and the input image (your puzzle input) to clean it up a little.
///
/// For example:
///
/// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
/// #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
/// .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
/// .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
/// .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
/// ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
/// ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
///
/// #..#.
/// #....
/// ##..#
/// ..#..
/// ..###
/// The first section is the image enhancement algorithm. It is normally given on a single line, but it has been wrapped to multiple lines in this example for legibility. The second section is the input image, a two-dimensional grid of light pixels (#) and dark pixels (.).
///
/// The image enhancement algorithm describes how to enhance an image by simultaneously converting all pixels in the input image into an output image. Each pixel of the output image is determined by looking at a 3x3 square of pixels centered on the corresponding input image pixel. So, to determine the value of the pixel at (5,10) in the output image, nine pixels from the input image need to be considered: (4,9), (4,10), (4,11), (5,9), (5,10), (5,11), (6,9), (6,10), and (6,11). These nine input pixels are combined into a single binary number that is used as an index in the image enhancement algorithm string.
///
/// For example, to determine the output pixel that corresponds to the very middle pixel of the input image, the nine pixels marked by [...] would need to be considered:
///
/// # . . # .
/// #[. . .].
/// #[# . .]#
/// .[. # .].
/// . . # # #
/// Starting from the top-left and reading across each row, these pixels are ..., then #.., then .#.; combining these forms ...#...#.. By turning dark pixels (.) into 0 and light pixels (#) into 1, the binary number 000100010 can be formed, which is 34 in decimal.
///
/// The image enhancement algorithm string is exactly 512 characters long, enough to match every possible 9-bit binary number. The first few characters of the string (numbered starting from zero) are as follows:
///
/// 0         10        20        30  34    40        50        60        70
/// |         |         |         |   |     |         |         |         |
/// ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
/// In the middle of this first group of characters, the character at index 34 can be found: #. So, the output pixel in the center of the output image should be #, a light pixel.
///
/// This process can then be repeated to calculate every pixel of the output image.
///
/// Through advances in imaging technology, the images being operated on here are infinite in size. Every pixel of the infinite output image needs to be calculated exactly based on the relevant pixels of the input image. The small input image you have is only a small region of the actual infinite input image; the rest of the input image consists of dark pixels (.). For the purposes of the example, to save on space, only a portion of the infinite-sized input and output images will be shown.
///
/// The starting input image, therefore, looks something like this, with more dark pixels (.) extending forever in every direction not shown here:
///
/// ...............
/// ...............
/// ...............
/// ...............
/// ...............
/// .....#..#......
/// .....#.........
/// .....##..#.....
/// .......#.......
/// .......###.....
/// ...............
/// ...............
/// ...............
/// ...............
/// ...............
/// By applying the image enhancement algorithm to every pixel simultaneously, the following output image can be obtained:
///
/// ...............
/// ...............
/// ...............
/// ...............
/// .....##.##.....
/// ....#..#.#.....
/// ....##.#..#....
/// ....####..#....
/// .....#..##.....
/// ......##..#....
/// .......#.#.....
/// ...............
/// ...............
/// ...............
/// ...............
/// Through further advances in imaging technology, the above output image can also be used as an input image! This allows it to be enhanced a second time:
///
/// ...............
/// ...............
/// ...............
/// ..........#....
/// ....#..#.#.....
/// ...#.#...###...
/// ...#...##.#....
/// ...#.....#.#...
/// ....#.#####....
/// .....#.#####...
/// ......##.##....
/// .......###.....
/// ...............
/// ...............
/// ...............
/// Truly incredible - now the small details are really starting to come through. After enhancing the original input image twice, 35 pixels are lit.
///
/// Start with the original input image and apply the image enhancement algorithm twice, being careful to account for the infinite size of the images. How many pixels are lit in the resulting image?
pub fn part1() {
    let data = read_from_data_dir("day20.txt").unwrap();
    let noise_algo = data.lines().nth(0).unwrap();
    let noise_algo = noise_algo.chars().map(|c| c).collect();
    let image = string_to_image(&data);
    let new_image = enhance_image(&image, &noise_algo);
    let double_enhanced = enhance_image(&new_image, &noise_algo);
    println!(
        "Day20:1 After enhancement, the number of on pixels is {} (4858,4886) too low ",
        count_on(&double_enhanced)
    );
}

fn count_on(image: &Vec<Vec<char>>) -> u32 {
    image
        .iter()
        .map(|row| row.iter().filter(|&&pixel| pixel == '#').count())
        .sum::<usize>() as u32
}

fn string_to_image(data: &str) -> Vec<Vec<char>> {
    data.lines()
        .skip(2)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn enhance_image(image: &Vec<Vec<char>>, noise_algo: &Vec<char>) -> Vec<Vec<char>> {
    let mut new_image = vec![vec!['.'; image[0].len()]; image.len()];
    for (row, sub_vector) in image.iter().enumerate() {
        for (col, pixel) in sub_vector.iter().enumerate() {
            new_image[row][col] = match get_binary_number(image, row, col) {
                Some(num) => noise_algo[num],
                None => *pixel,
            }
        }
    }

    new_image
}

// takes the row and col, finds the 9x9 square and create the binary number out of it
fn get_binary_number(image: &Vec<Vec<char>>, row: usize, col: usize) -> Option<usize> {
    if row == 0 || row == image.len() - 1 || col == 0 || col == image[0].len() - 1 {
        None
    } else {
        let mut binary = String::new();
        for r in row - 1..=row + 1 {
            for c in col - 1..=col + 1 {
                binary.push(binary_from_char(&image[r][c]));
            }
        }

        Some(isize::from_str_radix(&binary, 2).unwrap() as usize)
    }
}

fn binary_from_char(item: &char) -> char {
    return match item {
        '#' => '1',
        '.' => '0',
        _ => unreachable!(),
    };
}

pub fn part2() {}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_binary_number() {
        let image = vec![
            vec!['.', '.', '.'],
            vec!['#', '.', '.'],
            vec!['.', '#', '.'],
        ];
        assert_eq!(get_binary_number(&image, 1, 1).unwrap(), 34);
    }

    #[test]
    fn test_get_binary_number_2() {
        let image = vec![
            vec!['.', '.', '.'],
            vec!['#', '.', '.'],
            vec!['.', '#', '.'],
        ];
        assert_eq!(get_binary_number(&image, 0, 1), None);
    }

    #[test]
    fn test_enhance_image() {
        let image = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '#', '.', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];

        let correct_image = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '#', '.', '#', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '#', '.', '#', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '#', '#', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];

        let noise_str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
        let noise_algo = noise_str.chars().map(|c| c).collect();
        assert_eq!(enhance_image(&image, &noise_algo), correct_image);
        assert_eq!(count_on(&enhance_image(&image, &noise_algo)), 24);
    }

    #[test]
    fn test_enhance_image_2() {
        let correct_image = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '#', '.', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '#', '.', '.', '.', '#', '#', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '#', '#', '.', '#', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];

        let image = vec![
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '#', '.', '#', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '#', '.', '#', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '#', '#', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];

        let noise_str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
        let noise_algo = noise_str.chars().map(|c| c).collect();
        assert_eq!(enhance_image(&image, &noise_algo), correct_image);
        assert_eq!(count_on(&enhance_image(&image, &noise_algo)), 35);
    }

    #[test]
    fn test_count_on() {
        let image = vec![
            vec!['#', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.'],
            vec!['#', '#', '.', '.', '#'],
            vec!['.', '.', '#', '.', '.'],
            vec!['.', '.', '#', '#', '#'],
        ];

        assert_eq!(count_on(&image), 10);
    }
}
