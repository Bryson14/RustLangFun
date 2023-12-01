use crate::utils::read_file;
use std::env;

const DAY: &str = "01";

pub fn get_input() -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let input_path = current_dir.join(format!("input/day{}.txt", DAY));
    let input = read_file(&input_path.to_string_lossy());

    input
}