use std::fs;

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(&format!("Unable to read file: {}", file_path))
}

pub fn get_input(day: &str) -> String {
    let input_path = format!("../input/day{}.txt", day);
    let input = read_file(&input_path);

    input
}

pub fn get_example(day: &str, part: &str) -> (String, String) {
    let input_path = format!("../input/day{}_{}_example.txt", day, part);
    let input = read_file(&input_path);

    let answer_path = format!("../input/day{}_{}_example_answer.txt", day, part);
    let answer = read_file(&answer_path);

    (input, answer)
}
