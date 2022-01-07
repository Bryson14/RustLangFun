use crate::read_from_data_dir;
use std::fmt;

/// # --- Day 12: Passage Pathing ---
/// With your submarine's subterranean subsystems subsisting suboptimally, the only way you're getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only way to know if you've found the best path is to find all of them.
///
/// Fortunately, the sensors are still mostly working, and so you build a rough map of the remaining caves (your puzzle input). For example:
///
/// start-A
/// start-b
/// A-c
/// A-b
/// b-d
/// A-end
/// b-end
/// This is a list of how all of the caves are connected. You start in the cave named start, and your destination is the cave named end. An entry like b-d means that cave b is connected to cave d - that is, you can move between them.
///
/// So, the above cave system looks roughly like this:
/// ```text
///     start
///     /   \
/// c--A-----b--d
///     \   /
///      end
/// ```
/// Your goal is to find the number of distinct paths that start at start, end at end, and don't visit small caves more than once. There are two types of caves: big caves (written in uppercase, like A) and small caves (written in lowercase, like b). It would be a waste of time to visit any small cave more than once, but big caves are large enough that it might be worth visiting them multiple times. So, all paths you find should visit small caves at most once, and can visit big caves any number of times.
///
/// Given these rules, there are 10 paths through this example cave system:
///
/// start,A,b,A,c,A,end
/// start,A,b,A,end
/// start,A,b,end
/// start,A,c,A,b,A,end
/// start,A,c,A,b,end
/// start,A,c,A,end
/// start,A,end
/// start,b,A,c,A,end
/// start,b,A,end
/// start,b,end
/// (Each line in the above list corresponds to a single path; the caves visited by that path are listed in the order they are visited and separated by commas.)
///
/// Note that in this cave system, cave d is never visited by any path: to do so, cave b would need to be visited twice (once on the way to cave d and a second time when returning from cave d), and since cave b is small, this is not allowed.
///
/// Here is a slightly larger example:
///
/// dc-end
/// HN-start
/// start-kj
/// dc-start
/// dc-HN
/// LN-dc
/// HN-end
/// kj-sa
/// kj-HN
/// kj-dc
/// The 19 paths through it are as follows:
///
/// start,HN,dc,HN,end
/// start,HN,dc,HN,kj,HN,end
/// start,HN,dc,end
/// start,HN,dc,kj,HN,end
/// start,HN,end
/// start,HN,kj,HN,dc,HN,end
/// start,HN,kj,HN,dc,end
/// start,HN,kj,HN,end
/// start,HN,kj,dc,HN,end
/// start,HN,kj,dc,end
/// start,dc,HN,end
/// start,dc,HN,kj,HN,end
/// start,dc,end
/// start,dc,kj,HN,end
/// start,kj,HN,dc,HN,end
/// start,kj,HN,dc,end
/// start,kj,HN,end
/// start,kj,dc,HN,end
/// start,kj,dc,end
/// Finally, this even larger example has 226 paths through it:
///
/// fs-end
/// he-DX
/// fs-he
/// start-DX
/// pj-DX
/// end-zg
/// zg-sl
/// zg-pj
/// pj-he
/// RW-he
/// fs-DX
/// pj-RW
/// zg-RW
/// start-pj
/// he-WI
/// zg-he
/// pj-fs
/// start-RW
/// How many paths through this cave system are there that visit small caves at most once?
pub fn part1() {
    let data = read_from_data_dir("day12.txt").unwrap();
    let cave_map = create_map(data);
    let paths = find_unique_paths(cave_map);
}

fn create_map(data: String) -> NodeMap {
    let mut map: HashMap<String, Node> = HashMap::new();

    data.lines().for_each(|line| {
        let nodes: Vec<String> = line.split("-").map(|node| node.trim()).collect();
        assert_eq!(nodes.len(), 2);

        let node1 = &mut *map.entry(nodes[0]).or_insert(Node::new(nodes[0]));
        let _ = node1.add_destination(nodes[1]);

        let node2 = &mut *map.entry(nodes[1]).or_insert(Node::new(nodes[1]));
        let _ = node2.add_destination(nodes[0]);
    });

    assert_eq!(map.contains_key("start"), true);
    assert_eq!(map.contains_key("end"), true);

    NodeMap { map: map }
}

fn find_unique_paths(nodemap: NodeMap) -> Vec<String> {
    let mut path_list = Vec::new();
    unique_path(nodemap, "start".into(), "start".into(), &mut path_list);
    path_list
}

fn unique_path(nodemap: NodeMap, current_node: String, path: String, path_list: &mut Vec<String>) {
    let current_path = format!("{}->{}", path, current_node);
    if current_node == "end" {
        // got to end, record path
        if !path_list.contains(&current_path) {
            path_list.push(current_path);
        }
    } else if nodemap.map.values().all(|node| !node.can_visit()) {
        // stuck
        return;
    } else {
        // still going
        nodemap
            .map
            .values()
            .filter(|node| node.can_visit())
            .for_each(|node| {
                let map_copy = nodemap.clone();
                let node_copy = map_copy.map.get(&node.name).unwrap();
                *node_copy.visit();
                unique_path(map_copy, node.name, current_path, path_list);
            });
    }
}

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct NodeMap {
    map: HashMap<String, Node>,
}

#[derive(Debug, Copy, Clone)]
struct Node {
    destinations: Vec<String>,
    visits_left: usize,
    name: String,
}

impl Node {
    fn new(name: String) -> Node {
        // name is uppercase
        let mut visits_left = 1;
        if name == name.to_uppercase() {
            visits_left = 2;
        }
        Node {
            destinations: Vec::new(),
            visits_left: visits_left,
            name: String::from(name),
        }
    }

    fn add_destination(&mut self, name: String) {
        if !self.destinations.contains(&name) {
            self.destinations.push(name);
        }
    }

    fn visit(&mut self) {
        self.visits_left -= 1;
    }

    fn can_visit(&self) -> bool {
        self.visits_left > 0
    }
}

pub fn part2() {}

pub fn is_complete() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_map() {
        let data: String = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end"
            .to_string();

        let map = create_map(data);

        println!("map:{:?}", map);
        assert_eq!(1, 2);
    }
}
