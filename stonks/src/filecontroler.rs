use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

pub fn open_file(filename: &str) {
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
    println!("{}", contents);
}

pub fn append_file(filename: &str, to_append: &str) {
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

    let mut file = OpenOptions::new().append(true).open(file_loc).unwrap();

    if let Err(e) = file.write(to_append.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
