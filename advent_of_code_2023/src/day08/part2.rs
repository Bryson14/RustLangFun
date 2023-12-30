use crate::day08::{read_network, TwoPathNetwork};
/// # --- Part Two ---
/// The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!
///
/// What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.
///
/// After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.
///
/// For example:
///
/// LR
///
/// 11A = (11B, XXX)
/// 11B = (XXX, 11Z)
/// 11Z = (11B, XXX)
/// 22A = (22B, XXX)
/// 22B = (22C, 22C)
/// 22C = (22Z, 22Z)
/// 22Z = (22B, 22B)
/// XXX = (XXX, XXX)
/// Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:
///
/// Step 0: You are at 11A and 22A.
/// Step 1: You choose all of the left paths, leading you to 11B and 22B.
/// Step 2: You choose all of the right paths, leading you to 11Z and 22C.
/// Step 3: You choose all of the left paths, leading you to 11B and 22Z.
/// Step 4: You choose all of the right paths, leading you to 11Z and 22B.
/// Step 5: You choose all of the left paths, leading you to 11B and 22C.
/// Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
/// So, in this example, you end up entirely on nodes that end in Z after 6 steps.
///
/// Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
pub fn solve(input: &str) {
    todo!("Implement Day 8 Part 2. Its going in infinite cycles");
    let network = read_network(input);

    // get all the nodes that end in A
    let current_nodes: Vec<&str> = network
        .nodes
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| *node)
        .collect();
    println!("Current nodes: {:?}", current_nodes);
    let cycles = current_nodes
        .iter()
        .map(|node| calculate_steps_in_cycle(&network, node))
        .collect::<Vec<usize>>();

    // get the least common multiple of all the cycles,
    // fold the cycles into the least common multiple
    let steps = cycles
        .iter()
        .fold(1, |acc, cycle| least_common_multiple(acc, *cycle));

    // every 1000 steps print the number of steps

    println!("Steps to traverse the network simultaneously: {}", steps);
}

/// Gets the number of steps it takes for node to cycle to the node that ends in Z
fn calculate_steps_in_cycle(network: &TwoPathNetwork, node: &str) -> usize {
    let mut steps = 0;
    let mut current_node = node;

    // iterate over the instructions infinitely
    for instruction in network.instructions.chars().cycle() {
        if steps % 1000000 == 0 {
            println!(
                "Step: {}, Node: {}, Curr: {}, Instruction: {}",
                steps, node, current_node, instruction
            );
        }
        steps += 1;
        let node = network
            .nodes
            .get(node)
            .expect(&format!("Invalid node: {}", node));
        // println!("Node {}: {:?}, Instruction: {}", node_idx, node, instruction);
        match instruction {
            'L' => {
                current_node = node.left;
            }
            'R' => {
                current_node = node.right;
            }
            _ => panic!("Invalid instruction: {}", instruction),
        }

        // if all the nodes end in Z, we're done
        if current_node.ends_with("Z") {
            break;
        }
    }

    steps
}

fn greatest_common_factor(a: usize, b: usize) -> usize {
    let min_val = std::cmp::min(a, b);
    let max_val = std::cmp::max(a, b);
    if min_val == 0 {
        return max_val;
    }
    greatest_common_factor(min_val, max_val % min_val)
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * (b / greatest_common_factor(a, b))
}
