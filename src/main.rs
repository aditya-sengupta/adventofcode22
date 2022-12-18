use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate petgraph;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::algo::dijkstra;
use std::collections::{HashMap,HashSet};
extern crate regex;
use regex::Regex;
extern crate bimap;
use bimap::{BiMap,BiHashMap};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // day16_01("day16");
    day16_02("day16_test");
}
struct Network {
    flow_rates: HashMap<NodeIndex, i32>,
    name_inds: BiHashMap<String, NodeIndex>,
    dists: HashMap<(NodeIndex, NodeIndex), i32>
}


fn all_distances<N, E>(graph: &Graph<N, E>) -> HashMap<(NodeIndex, NodeIndex), i32> {
    let mut distances: HashMap<(NodeIndex, NodeIndex), i32> = HashMap::new();
    for start in graph.node_indices() {
        for end in graph.node_indices() {
            let pair = (start, end);
            if !distances.contains_key(&pair) {
                let dj = dijkstra(&graph, start, Some(end), |_| 1);
                for (dest, d) in dj {
                    distances.insert((start, dest), d);
                    distances.insert((dest, start), d);
                }
            }
        }
    }
    distances
}

fn day16_01(fname: &str) {
    let (network, candidate_valves) = make_network(fname);
    let aa = *network.name_inds.get_by_left("AA").unwrap();
    println!("{:?}", traverse(aa, &candidate_valves, 0, 30, &network))
}

fn day16_02(fname: &str) {
    let (network, candidate_valves) = make_network(fname);
    let aa = *network.name_inds.get_by_left("AA").unwrap();
    println!("{:?}", traverse_elephant(aa, aa, 0, 0, &candidate_valves, 0, 26, &network))
}

fn make_network(fname: &str) -> (Network, HashSet<NodeIndex>) {
    let re = Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnel(?:s|) lead(?:s|) to valve(?:s|) ((?:\w\w(?:, |))+)").unwrap();
    let mut name_inds = BiMap::<String, NodeIndex>::new();
    let mut flow_rates = HashMap::<NodeIndex, i32>::new();
    let mut candidate_valves = HashSet::<NodeIndex>::new();
    let mut graph = Graph::<String, ()>::new();
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            let ip = line.unwrap();
            let cap = re.captures(ip.as_str()).unwrap();
            let node = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let flow_rate = cap.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            let idx = match name_inds.contains_left(&node) {
                true => *name_inds.get_by_left(&node).unwrap(),
                false => {
                    let idxt = graph.add_node(node.clone());
                    name_inds.insert(node.clone(), idxt);
                    idxt
                }
            };
            flow_rates.insert(idx, flow_rate);
            if flow_rate > 0 {
                candidate_valves.insert(idx);
            }
            name_inds.insert(node.clone(), idx);
            let adjs: Vec<&str> = cap.get(3).unwrap().as_str().trim().split(",").map(|m| m.trim()).collect();
            for a in adjs {
                let idxa = match name_inds.contains_left(a) {
                    true => *name_inds.get_by_left(a).unwrap(),
                    false => {
                        let a_s = a.to_string();
                        let idxt = graph.add_node(a.to_string());
                        name_inds.insert(a_s, idxt);
                        idxt
                    }
                };
                graph.add_edge(idx, idxa, ());
            }
        }
    }
    let dists = all_distances(&graph);
    return (Network { flow_rates, name_inds, dists }, candidate_valves)
}

fn traverse(
    curr_node: NodeIndex,
    candidate_valves: &HashSet<NodeIndex>,
    score: i32,
    time_left: i32,
    network: &Network,
) -> i32 {
    if (time_left <= 0) || (candidate_valves.len() == 0) { 
        return score;
    } else {
        // try moving to each closed valve
        let mut new_candidate_valves = candidate_valves.clone();
        new_candidate_valves.remove(&curr_node);
        let reward_here = *network.flow_rates.get(&curr_node).unwrap() * time_left;
        let mut cost_to_go = score + reward_here;
        for n in new_candidate_valves.clone() {
            
            let d = *network.dists.get(&(curr_node, n)).unwrap();
            let path_reward = traverse(
                n,
                &new_candidate_valves,
                score + reward_here,
                time_left - d - 1,
                network
            );
            cost_to_go = cost_to_go.max(path_reward);
        }
        return cost_to_go
    }
}

/*
pseudocode for traversal with an elephant

if there's no time or all the valves are open, return the score
otherwise, check your node and the elephant's, AND if you're actually at your destination
if you are at a node, take a minute to open the valve (decrement the elephant's distance to destination), then run through the set of next destinations
of these, rule out the elephant's node
do a recursive call with your new target node, the elephant's target node, the new score after the valve you just opened, and the time left, which is the min of the distance to your new target and the elephant's

*/

fn traverse_elephant(
    your_node: NodeIndex,
    elephant_node: NodeIndex,
    your_dist: i32,
    elephant_dist: i32,
    candidate_valves: &HashSet<NodeIndex>,
    score: i32,
    time_left: i32,
    network: &Network,
) -> i32 {
    if (time_left <= 0) || (candidate_valves.len() == 0) { 
        return score;
    } else {
        let step = your_dist.min(elephant_dist);
        let time_left = time_left - step;
        let nyour_dist = your_dist - step;
        let nelephant_dist = elephant_dist - step;
        let mut cost_to_go = score;
        if nyour_dist <= 0 {
            let mut new_candidate_valves = candidate_valves.clone();
            new_candidate_valves.remove(&your_node);
            let reward_here = *network.flow_rates.get(&your_node).unwrap() * time_left;
            cost_to_go = cost_to_go.max(score + reward_here);
            for n in new_candidate_valves.clone() {
                let d = *network.dists.get(&(your_node, n)).unwrap();
                let path_reward = traverse_elephant(
                    n,
                    elephant_node,
                    d,
                    nelephant_dist - 1,
                    &new_candidate_valves,
                    score + reward_here,
                    time_left - 1,
                    network
                );
            cost_to_go = cost_to_go.max(path_reward);
            }
        }
        if nelephant_dist <= 0 {
            let mut new_candidate_valves = candidate_valves.clone();
            new_candidate_valves.remove(&elephant_node);
            let reward_here = *network.flow_rates.get(&elephant_node).unwrap() * time_left;
            cost_to_go = cost_to_go.max(score + reward_here);
            for n in new_candidate_valves.clone() {
                let d = *network.dists.get(&(elephant_node, n)).unwrap();
                let path_reward = traverse_elephant(
                    n,
                    elephant_node,
                    nyour_dist - 1,
                    d,
                    &new_candidate_valves,
                    score + reward_here,
                    time_left - 1,
                    network
                );
            cost_to_go = cost_to_go.max(path_reward);
            }
        }
        return cost_to_go
    }
}
