use std::env::current_dir;
use std::fs::read_to_string;

pub fn read_data(filename: &str) -> String {
    let path = current_dir().unwrap().join("data").join(filename);
    let path_str = path.display().to_string();
    let Ok(data_str) = read_to_string(path) else {
        println!("[Error] No file found at: {path_str}");
        panic!("No file found");
    };
    data_str
}
