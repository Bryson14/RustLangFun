use std::env::current_dir;
use std::fs::read_to_string;

pub fn read_data(filename: &str) -> String {
    let path = current_dir().unwrap().join("data").join(filename);
    let path_str = path.display().to_string();
    let Ok(data_str) = read_to_string(path) else {
        panic!("[Error] No file found at: {path_str}");
    };
    data_str
}
