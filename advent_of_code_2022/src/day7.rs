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
    let mut parent_node = parse_input(data.as_str());
    parent_node.sum_small_dirs(limit);
    parent_node.calc_size();
    println!(
        "{DAY} Total size of the filesystem ignoring small dirs is {}",
        parent_node.size
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
        }
    }

    // get the child by looking into the struct of the parent and getting the strings
    fn get_child_path(&self, parent_path: &str, child_name: &str) -> Option<&str> {
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
        if parent_path == "" && child_name == "/" {
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
    path.split("/").collect()
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
                .expect(&format!(
                    "Could not find child {} in path {}",
                    name, curr_node
                ))
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

///
/// SPLIT between methods
///

fn concat_path(parent_path: &str, child_name: &str, child_level: usize) -> String {
    let mut path = String::from(child_name);
    if child_level > 1 {
        // to stop the `//` in the paths
        // top level path
        path.insert_str(0, "/");
    }
    path.insert_str(0, parent_path);
    path
}

impl TreeNode {
    fn new(name: String, level: usize, parent_path: &str) -> Self {
        let path = concat_path(parent_path, name.as_str(), level);
        let list = path.split("/").collect::<Vec<&str>>();
        assert!(name == "/" || list[level] == name);
        TreeNode {
            name,
            level,
            path: path,
            size: 0,
            children: Vec::new(),
        }
    }

    /// calculates the current nodes size
    fn calc_size(&mut self) {
        let size = self.children.iter_mut().map(|c| c.size_helper()).sum();
        self.size = size;
    }

    /// find the nodes size and then bubble up its size
    fn size_helper(&mut self) -> u128 {
        if self.is_leaf() {
            return self.size;
        }
        let children_size = self.children.iter_mut().map(|c| c.size_helper()).sum();
        self.size = children_size;
        children_size
    }

    fn get_child(&mut self, child_path: &str) -> Option<&mut TreeNode> {
        self.children.iter_mut().find(|c| c.name == child_path)
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_dir(&self) -> bool {
        !self.children.is_empty()
    }

    fn sum_small_dirs(&self, limit: u128) -> u128 {
        // this is a leaf
        if self.is_leaf() {
            return 0;
        }
        // This is a large dir
        if self.size > limit {
            return self.children.iter().map(|c| c.sum_small_dirs(limit)).sum();
        }
        // this is a dir below the limit
        self.size
    }

    fn is_node_by_path(&self, path: &str) -> bool {
        self.path == path
    }

    /// starting at the top, assuming there is no duplicate file or dir names lmao
    fn find_parent(
        &mut self,
        child_name: &str,
        child_path: &str,
        parent_level: usize,
    ) -> Option<&mut TreeNode> {
        // too deep into the tree
        let level = self.level;
        if level > parent_level {
            return None;
        }
        // it is the parent!
        let child = self.get_child(child_name);
        if level == parent_level && child.is_some() && child.unwrap().is_node_by_path(child_path) {
            return Some(self);
        }
        // the parent is possibly below this node
        if self.level < parent_level {
            for child in self.children.iter_mut() {
                let r = child.find_parent(child_name, child_path, parent_level);
                if r.is_some() {
                    return r;
                }
            }
            return None;
        }
        None
    }
}

fn parse_input(data: &str) -> TreeNode {
    let mut parent_node = TreeNode::new("/".to_owned(), 0, "");
    let mut curr_node: &mut TreeNode = &mut parent_node;
    let re_cd = Regex::new(r"^\$\s?cd\s(.+)$").unwrap();
    let re_dir = Regex::new(r"^dir\s(.+)$").unwrap();
    let re_file = Regex::new(r"^(\d+)\s(.+)$").unwrap();

    for (i, line) in data.lines().enumerate() {
        if i == 677 {
            print!("here");
        }
        if line.starts_with("$") {
            // command
            if line.contains("ls") {
                println!("{} - {}", i + 1, curr_node.path);
                // listing out current node
            } else if line.contains("cd") {
                if line.contains("..") {
                    let child_name = curr_node.name.clone();
                    let child_path = curr_node.path.clone();
                    let level = curr_node.level;
                    curr_node = parent_node
                        .find_parent(&child_name, &child_path, level - 1)
                        .expect(&format!("Could not find parent node of {child_name}"));
                    assert!(curr_node.is_dir());
                } else {
                    let cap = re_cd.captures(line).unwrap();
                    let name = cap.get(1).expect("no cd char found").as_str().trim();

                    if name != "/" {
                        let parent = curr_node.name.clone();
                        let parent_path = curr_node.path.clone();
                        let new_node: &mut TreeNode = curr_node.get_child(name).expect(&format!(
                            "Could not find child node {} of parent {}. Line: {}",
                            name,
                            parent_path,
                            i + 1
                        ));
                        curr_node = new_node;
                    }
                }
            } else {
                unreachable!();
            }
        } else if line.starts_with("dir") {
            // directory node
            let cap = re_dir
                .captures(line)
                .expect("Re dir could not match on line");
            let name = cap.get(1).expect("no dir name found").as_str().trim();
            let mut node = TreeNode::new(name.to_owned(), curr_node.level + 1, &curr_node.path);
            curr_node.children.push(node);
        } else if re_file.is_match(line) {
            // file with byte size
            let cap = re_file
                .captures(line)
                .expect(" Re_file could not match on line");
            let size = cap
                .get(1)
                .expect("no byte size found")
                .as_str()
                .parse::<u128>()
                .expect("Couldn't parse file size");
            let name = cap.get(2).expect("no filename found").as_str().trim();
            let mut node = TreeNode::new(name.to_owned(), curr_node.level + 1, &curr_node.path);
            node.size = size;
            curr_node.children.push(node);
        } else {
            unreachable!();
        }
    }
    parent_node.calc_size();
    parent_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        let data = "$ cd /\n$ls \ndir a\n123 b\n";
        let node = parse_input(data);

        assert_eq!(node.size, 123);
        assert_eq!(node.name, "/");
        assert_eq!(
            node.children,
            vec![
                TreeNode {
                    name: "a".to_string(),
                    level: 1,
                    path: "/a".to_string(),
                    size: 0,
                    children: Vec::new()
                },
                TreeNode {
                    name: "b".to_string(),
                    level: 1,
                    path: "/b".to_string(),
                    size: 123,
                    children: Vec::new()
                }
            ]
        );
    }

    #[test]
    fn test_parse_2() {
        let data = "$ cd /\n$ ls \ndir a\n10 b\n$ cd a\n$ ls \ndir c \n5 e";
        let node = parse_input(data);

        assert_eq!(node.size, 15);
        assert_eq!(node.name, "/");
        assert_eq!(
            node.children,
            vec![
                TreeNode {
                    name: "a".to_string(),
                    level: 1,
                    path: "/a".to_string(),
                    size: 5,
                    children: vec![
                        TreeNode {
                            name: "c".to_string(),
                            level: 2,
                            path: "/a/c".to_string(),
                            size: 0,
                            children: Vec::new()
                        },
                        TreeNode {
                            name: "e".to_string(),
                            level: 2,
                            path: "/a/e".to_string(),
                            size: 5,
                            children: Vec::new()
                        }
                    ]
                },
                TreeNode {
                    name: "b".to_string(),
                    level: 1,
                    path: "/b".to_string(),
                    size: 10,
                    children: Vec::new()
                }
            ]
        );
    }
}
