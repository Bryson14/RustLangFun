use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn num_of_increases() {
    let data = open_file("message.txt");
    let nums: Vec<i32> = data
        .split("\n")
        .map(|n| n.parse::<i32>().expect("Error parsing"))
        .collect();
    let mut count = 0;
    for (i, num) in nums.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if num > &nums[i - 1] {
            count += 1;
        }
    }

    println!("count: {}", count);
}

fn open_file(filename: &str) -> String {
    let curr_exe = env::current_exe().unwrap();
    let curr_dir = curr_exe
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let file_loc = curr_dir.join("data").join(filename);
    println!("file: {:?}", file_loc);
    let mut file = File::open(file_loc).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
