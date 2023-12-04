use std::fs;
use std::path::{Path, PathBuf};

pub fn read_file(file_path: &Path) -> String {
    // Check if the file exists and is a file
    if let Ok(metadata) = fs::metadata(file_path) {
        if metadata.is_file() {
            return fs::read_to_string(file_path)
                .expect(&format!("Unable to read file: {:?}", file_path.display()));
        } else {
            panic!("Specified path is not a file: {:?}", file_path);
        }
    } else {
        panic!("File not found: {:?}", file_path);
    }
}

pub fn get_input(day: &str) -> String {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    // assert that current path is the root of the project advent_of_code_2023
    if !current_dir.ends_with("advent_of_code_2023") {
        panic!(
            "Current directory is not the root of the project: {:?}",
            current_dir
        );
    }

    let input_path = Path::new("input").join(format!("day{}.txt", day));
    let absolute_path = input_path
        .canonicalize()
        .expect("Unable to get absolute path");
    let input = read_file(&absolute_path);

    input
}

pub fn get_example(day: &str, part: &str) -> (String, String) {
    // let test = Path::new("../input");
    // let absolute_path = test.canonicalize().expect("Unable to get absolute path");
    // println!("Absolute path: {:?}", absolute_path);

    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    // assert that current path is the root of the project advent_of_code_2023
    if !current_dir.ends_with("advent_of_code_2023") {
        panic!(
            "Current directory is not the root of the project: {:?}",
            current_dir
        );
    }

    let input_path = Path::new("input").join(format!("day{}_{}_example.txt", day, part));
    let absolute_path = input_path
        .canonicalize()
        .expect("Unable to get absolute path");
    let input = read_file(&absolute_path);

    let answer_path = Path::new("input").join(format!("day{}_{}_example_answer.txt", day, part));
    let absolute_path = answer_path
        .canonicalize()
        .expect("Unable to get absolute path");
    let answer = read_file(&absolute_path);

    (input, answer)
}
