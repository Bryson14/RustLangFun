#![allow(unused)]
use crate::utils::read_data;
use regex::Regex;

const FILE: &str = "day7.txt";
const DAY: &str = "{{ DAY 7 }}";

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

impl TreeNode {
    fn new(name: String, level: usize, parent_path: &str) -> Self {
        let mut path = name.clone();
        if name != "/" {
            // top level path
            path.insert_str(0, "/");
        }
        path.insert_str(0, parent_path);
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

    fn get_child(&mut self, name: &str) -> Option<&mut TreeNode> {
        self.children.iter_mut().find(|c| c.name == name)
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
        if line.starts_with("$") {
            // command
            if line.contains("ls") {
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
                    path: "//a".to_string(),
                    size: 0,
                    children: Vec::new()
                },
                TreeNode {
                    name: "b".to_string(),
                    level: 1,
                    path: "//b".to_string(),
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
                    path: "//a".to_string(),
                    size: 5,
                    children: vec![
                        TreeNode {
                            name: "c".to_string(),
                            level: 2,
                            path: "//a/c".to_string(),
                            size: 0,
                            children: Vec::new()
                        },
                        TreeNode {
                            name: "e".to_string(),
                            level: 2,
                            path: "//a/e".to_string(),
                            size: 5,
                            children: Vec::new()
                        }
                    ]
                },
                TreeNode {
                    name: "b".to_string(),
                    level: 1,
                    path: "//b".to_string(),
                    size: 10,
                    children: Vec::new()
                }
            ]
        );
    }
}
