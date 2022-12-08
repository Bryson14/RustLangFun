#![allow(unused)]
use std::collections::HashMap;

use crate::utils::read_data;
use itertools::concat;
use regex::Regex;

const FILE: &str = "day7.txt";
const DAY: &str = "{{ DAY 7 }}";
const SEP: &str = "/";

/// --- Day 7: No Space Left On Device ---
/// You browse around the filesystem to assess the situation and
/// save the resulting terminal output (your puzzle input).
/// Find all of the directories with a total size of at most 100000.
/// What is the sum of the total sizes of those directories?
pub fn part1() {
    let data = read_data(FILE);
    let limit = 100000;
    let mut filesys = parse_data_v2(data.as_str());
    let ans = filesys.sum_small_dirs("~", limit);
    println!(
        "{DAY} Total size of the filesystem ignoring small dirs is {}",
        ans
    );
}

pub fn part2() {
    let data = read_data(FILE);
}

/// A tree map that only stores one-way downward references to reduce the headache of Rc<RefCell<TreeNode>>
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    name: String,
    level: usize,
    path: String,
    size: u128,
    children: Vec<TreeNode>,
}

struct FileSystem {
    files: HashMap<String, Contents>,
    size: usize,
}

struct Contents {
    size: usize,
    children: Vec<String>,
    parent: String,
}

impl Contents {
    // size is the byte size of the file or dir

    fn new() -> Self {
        Contents {
            size: 0,
            children: Vec::new(),
            parent: "".to_string(),
        }
    }

    fn is_dir(&self) -> bool {
        !self.children.is_empty()
    }

    fn is_file(&self) -> bool {
        self.children.is_empty()
    }
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            files: HashMap::new(),
            size: 0,
        }
    }

    fn sum_small_dirs(&self, path: &str, limit: usize) -> usize {
        todo!()
    }

    // get the child by looking into the struct of the parent and getting the strings
    fn get_child_path(&mut self, parent_path: &str, child_name: &str) -> Option<&str> {
        if let Some(contents) = self.files.get(parent_path) {
            let child_path = join_path(parent_path, child_name);
            return Some(contents.children.iter().find(|&p| p == child_name).unwrap());
        }
        None
    }

    // gets the
    fn get_parent_path(&self, this_path: &str) -> String {
        let mut parsed = parse_path(this_path);
        let _ = parsed.pop();
        parsed.join(SEP)
    }

    fn add_dir(&mut self, parent_path: &str, child_name: &str) {
        self.add_child(parent_path, child_name, 0);
    }

    fn add_file(&mut self, parent_path: &str, child_name: &str, child_size: usize) {
        self.add_child(parent_path, child_name, child_size);
    }

    fn add_child(&mut self, parent_path: &str, child_name: &str, child_size: usize) {
        let mut second_path = String::new();
        let mut found = false;
        // top level dir
        if parent_path.is_empty() && child_name == "/" {
            self.files.insert("~".to_string(), Contents::new());
            return;
        }
        if let Some(parent) = self.files.get(parent_path) {
            found = true;
            let child_path = join_path(parent_path, child_name);
            second_path = child_path.clone();
            let child_contents = Contents {
                size: child_size,
                children: Vec::new(),
                parent: parent_path.to_string(),
            };
            self.files.insert(child_path, child_contents);
        }
        if found {
            let mut c = self.files.remove(parent_path).unwrap();
            c.children.push(second_path);
            self.files.insert(parent_path.to_string(), c);

            let parsed = parse_path(parent_path);
            for i in (0..parsed.len()).rev() {
                let p = &parsed[0..=i].join(SEP);
                let mut c = self
                    .files
                    .remove(p)
                    .unwrap_or_else(|| panic!("{} : Cant remove {}", i, p));
                c.size += child_size;
                self.files.insert(p.to_string(), c);
            }
        }
    }
}

fn join_path(parent_path: &str, child_name: &str) -> String {
    let mut path = parent_path.to_string();
    path.push_str(SEP);
    path.push_str(child_name);
    path
}

fn parse_path(path: &str) -> Vec<&str> {
    path.split('/').collect()
}

fn parse_data_v2(data: &str) -> FileSystem {
    let mut filesys = FileSystem::new();
    let mut curr_node: String = "".into();
    let re_cd = Regex::new(r"^\$\s?cd\s(.+)$").unwrap();
    let re_dir = Regex::new(r"^dir\s(.+)$").unwrap();
    let re_file = Regex::new(r"^(\d+)\s(.+)$").unwrap();

    for (i, line) in data.lines().enumerate() {
        if line.starts_with("$ cd ..") {
            curr_node = filesys.get_parent_path(&curr_node);
        } else if line.starts_with("$ ls") {
            // listing out the contents of curr_node
        } else if line.starts_with("$ cd") {
            let cap = re_cd.captures(line).unwrap();
            let name = cap.get(1).expect("no cd char found").as_str().trim();
            curr_node = filesys
                .get_child_path(&curr_node, name)
                .unwrap_or_else(|| {
                    panic!(
                        "{}: Could not find child {} in path {}",
                        176 + i,
                        name,
                        curr_node
                    )
                })
                .to_string();
        } else if line.starts_with("dir") {
            let cap = re_dir
                .captures(line)
                .expect(" Re_file could not match on line");
            let name = cap.get(1).expect("no filename found").as_str().trim();
            filesys.add_dir(&curr_node, name);
        } else if re_file.is_match(line) {
            let cap = re_file
                .captures(line)
                .expect(" Re_file could not match on line");
            let size = cap
                .get(1)
                .expect("no byte size found")
                .as_str()
                .parse::<usize>()
                .expect("Couldn't parse file size");
            let name = cap.get(2).expect("no filename found").as_str().trim();
            filesys.add_file(&curr_node, name, size);
        } else {
            unreachable!();
        }
    }
    filesys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {}
}
