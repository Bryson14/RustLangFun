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
    let (cave_map, lookup_name_table) = create_map(data);
    let paths = find_unique_paths(cave_map, lookup_name_table);
    println!(
        "Day12:1. The number of unique paths is {}",
        paths.iter().count()
    );
}

fn create_map(data: String) -> (NodeMap, HashMap<usize, String>) {
    let mut map: HashMap<usize, Node> = HashMap::new();
    let mut lookup_table: HashMap<usize, String> = HashMap::new();
    let mut id = 1;

    data.lines().for_each(|line| {
        let node_names: Vec<String> = line
            .split("-")
            .map(|node| node.trim().to_string())
            .collect();
        assert_eq!(node_names.len(), 2);

        // get or make ID
        let (mut id_node_0, mut id_node_1): (usize, usize) = (0, 0);
        if let Some((id0, _node0)) = lookup_table
            .iter()
            .filter(|(&_k, v)| &&node_names[0] == v)
            .nth(0)
        {
            id_node_0 = *id0;
        } else {
            id_node_0 = id;
            id += 1;
            lookup_table.insert(id_node_0, node_names[0].clone());
        }

        if let Some((id1, _node1)) = lookup_table
            .iter()
            .filter(|(&_k, v)| &&node_names[1] == v)
            .nth(0)
        {
            id_node_1 = *id1;
        } else {
            id_node_1 = id;
            id += 1;
            lookup_table.insert(id_node_1, node_names[1].clone());
        }

        // enter into nodemap

        let node1 = &mut *map
            .entry(id_node_0)
            .or_insert(Node::new(node_names[0].clone(), id_node_0));
        let _ = node1.add_destination(id_node_1);

        let node2 = &mut *map
            .entry(id_node_1)
            .or_insert(Node::new(node_names[1].clone(), id_node_1));
        let _ = node2.add_destination(id_node_0);
    });

    (NodeMap { map: map }, lookup_table)
}

fn find_unique_paths(nodemap: NodeMap, lookup_name_table: HashMap<usize, String>) -> Vec<String> {
    let start_node: usize = lookup_name_table
        .iter()
        .filter(|(_k, v)| v == &&"start".to_string())
        .map(|(&k, _v)| k)
        .nth(0)
        .unwrap();
    let end_node: usize = lookup_name_table
        .iter()
        .filter(|(_k, v)| v == &&"end".to_string())
        .map(|(&k, _v)| k)
        .nth(0)
        .unwrap();
    unique_path(nodemap, start_node, end_node)
}

fn unique_path(mut nodemap: NodeMap, current_id: usize, end_node: usize) -> Vec<String> {
    // this nodemap is mine to alter
    (*nodemap.map.get_mut(&current_id).unwrap()).visit();
    if current_id == end_node {
        // got to end, record path
        vec![current_id.to_string()]
    } else if (*nodemap.map.get(&current_id).unwrap())
        .destinations
        .iter()
        .map(|id| nodemap.map.get(id).unwrap())
        .all(|node| !node.can_visit())
    {
        // stuck
        Vec::new()
    } else {
        // still going
        let mut paths = Vec::new();
        (*nodemap.map.get(&current_id).unwrap())
            .destinations
            .iter()
            .map(|id| (id, nodemap.map.get(id).unwrap()))
            .filter(|(_id, node)| node.can_visit())
            .for_each(|(&id, _node)| {
                let map_copy = nodemap.clone();
                let paths_produced = unique_path(map_copy, id, end_node);
                paths_produced.iter().for_each(|s| {
                    paths.push(format!("{}->{}", current_id, s));
                });
            });

        paths
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct NodeMap {
    map: HashMap<usize, Node>,
}

#[derive(Debug, Clone)]
struct Node {
    destinations: Vec<usize>,
    visits_left: usize,
    id: usize,
}

impl Node {
    fn new(name: String, id: usize) -> Node {
        // name is uppercase
        let mut visits_left = 1;
        if name == name.to_uppercase() {
            visits_left = 2;
        }
        Node {
            destinations: Vec::new(),
            visits_left: visits_left,
            id: id,
        }
    }

    fn add_destination(&mut self, id: usize) {
        if !self.destinations.contains(&id) {
            self.destinations.push(id);
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
        start-A
        A-end
        b-end"
            .to_string();

        let (nodemap, lookup_table) = create_map(data);

        println!("map:{:?}", nodemap);
        println!("lookup:{:?}", lookup_table);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_find_paths() {
        let data: String = "start-A
        start-b
        start-A
        A-end
        b-end"
            .to_string();

        let (cave_map, lookup_table) = create_map(data);

        let paths = find_unique_paths(cave_map, lookup_table);
        assert_eq!(paths, vec!["1->2->4", "1->3->4"]);
    }

    #[test]
    fn test_find_paths_2() {
        let data: String = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end"
            .to_string();

        let (cave_map, lookup_table) = create_map(data);

        let mut paths = find_unique_paths(cave_map, lookup_table);
        println!("paths: {:?}", paths);
        let mut correct = vec![
            "1->2->3->2->4->2->6",
            "1->2->3->2->6", //
            "1->2->3->6",    //
            "1->2->4->2->3->2->6",
            "1->2->4->2->3->6", //
            "1->2->4->2->6",    //
            "1->2->6",          //
            "1->3->2->4->2->6", //
            "1->3->2->6",       //
            "1->3->6",          //
        ];
        correct.sort_unstable();
        paths.sort_unstable();
        assert!(paths.iter().all(|s| correct.contains(&&s[..])) && paths.len() == correct.len());
    }
}
