use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate petgraph;
use petgraph::Graph;
use petgraph::graph::{NodeIndex};
use std::collections::HashMap;
extern crate regex;
use regex::Regex;
extern crate bimap;
use bimap::BiMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day16_01("day16_test");
}

fn day16_01(fname: &str) {
    let re = Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnel(?:s|) lead(?:s|) to valve(?:s|) ((?:\w\w(?:, |))+)").unwrap();
    let mut name_inds = BiMap::<String, NodeIndex>::new();
    let mut flow_rates = HashMap::<String, i32>::new();
    let mut graph = Graph::<String, (), petgraph::Undirected>::new_undirected();
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            let ip = line.unwrap();
            let cap = re.captures(ip.as_str()).unwrap();
            let node = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let flow_rate = cap.get(2).map_or(0, |m| m.as_str().parse::<i32>().unwrap());
            flow_rates.insert(node.to_string(), flow_rate);
            let idx = graph.add_node(node.clone());
            name_inds.insert(node, idx);
            let adjs: Vec<&str> = cap.get(3).unwrap().as_str().trim().split(",").collect();
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

}