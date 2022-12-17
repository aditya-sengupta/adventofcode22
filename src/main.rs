use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate petgraph;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
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
    day16_01("day16");
}
struct Network {
    graph: Graph<String, ()>,
    nnodes: usize,
    flow_rates: HashMap<NodeIndex, i32>,
    name_inds: BiHashMap<String, NodeIndex>
}

fn day16_01(fname: &str) {
    let network = make_network(fname);
    let aa = *network.name_inds.get_by_left("AA").unwrap();
    let mut open_valves = HashSet::<NodeIndex>::new();
    for (v, f) in &network.flow_rates {
        if *f == 0 { open_valves.insert(*v); }
    }
    let score = traverse(aa, open_valves, HashSet::<NodeIndex>::new(), 0, 0, &network);
    println!("{}", score);
}

fn make_network(fname: &str) -> Network {
    let re = Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnel(?:s|) lead(?:s|) to valve(?:s|) ((?:\w\w(?:, |))+)").unwrap();
    let mut name_inds = BiMap::<String, NodeIndex>::new();
    let mut flow_rates = HashMap::<NodeIndex, i32>::new();
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
    let nnodes = graph.node_count() as usize;
    return Network { graph, nnodes, flow_rates, name_inds }
}

fn traverse(
    curr_node: NodeIndex,
    open_valves: HashSet<NodeIndex>,
    recent_visits: HashSet<NodeIndex>,
    score: i32,
    time: i32,
    network: &Network
) -> i32 {
    const MAX_TIME: i32 = 30;
    if (time == MAX_TIME) || (open_valves.len() == network.nnodes) { 
        return score 
    } else {
        let mut cost_to_go = 0;
        if !open_valves.contains(&curr_node) {
            // try opening the valve you're at
            let flow_rate = *network.flow_rates.get(&curr_node).unwrap();
            if flow_rate > 0 {
                let mut new_open_valves = open_valves.clone();
                new_open_valves.insert(curr_node);
                let path_reward = traverse(
                    curr_node, 
                    new_open_valves, 
                    HashSet::<NodeIndex>::new(), 
                    score + (MAX_TIME - time - 1) * flow_rate,
                    time + 1,
                    network
                );
                cost_to_go = cost_to_go.max(path_reward);
            }
        }
        // try moving to each neighbour except the ones that you've visited recently
        for n in network.graph.neighbors(curr_node) {
            if !recent_visits.contains(&n) {
                let mut new_recent_visits = recent_visits.clone();
                new_recent_visits.insert(curr_node);
                let path_reward = traverse(
                    n,
                    open_valves.clone(),
                    new_recent_visits,
                    score,
                    time + 1,
                    network
                );
                cost_to_go = cost_to_go.max(path_reward);
            }
        }
        return cost_to_go
    }
}
