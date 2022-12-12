use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate priority_queue;
use priority_queue::PriorityQueue;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day12_01();
}

fn neighbors(grid: &Vec<Vec<u8>>, vis: &Vec<Vec<bool>>, current: (usize, usize), xm: usize, ym: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if current.0 != 0 { neighbors.push((current.0 - 1, current.1)) }
    if current.0 != ym - 1 { neighbors.push((current.0 + 1, current.1)) }
    if current.1 != 0 { neighbors.push((current.0, current.1 - 1)) }
    if current.1 != xm - 1 { neighbors.push((current.0, current.1 + 1)) }
    let c = grid[current.0][current.1] as u8;
    neighbors.retain(|n| c > grid[n.0][n.1] || grid[n.0][n.1] - c <= 1);
    neighbors.retain(|n| !vis[n.0][n.1]);
    neighbors
}

fn day12_01() {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut pq: PriorityQueue<(usize, usize), usize> = PriorityQueue::new();
    let s = 'S' as u8;
    let e = 'E' as u8;
    if let Ok(lines) = read_lines("./day12_test") {
        let mut i = 0;
        for line in lines { 
            if let Ok(ip) = line {
                grid.push(vec![]);
                for (j, b) in ip.as_bytes().iter().enumerate() {
                    let c: u8 = *b;
                    if c == s { 
                        start = (i, j); grid[i].push('a' as u8);
                    } else if c == e { 
                        end = (i, j); grid[i].push('z' as u8)
                    } else {
                        grid[i].push(c);
                    }
                }
            }
            i += 1;
        }
    }
    let xm = grid[0].len();
    let ym = grid.len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; xm]; ym];
    let mut distance: Vec<Vec<usize>> = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    distance[start.0][start.1] = 0;
    let mut current = start;
    while current != end {
        let currdist = distance[current.0][current.1];
        let neighbors = neighbors(&grid, &visited, current, xm, ym);
        for n in neighbors {
            let d = distance[n.0][n.1].min(currdist + 1);
            distance[n.0][n.1] = d;
            pq.push(n, d);
        }
        visited[current.0][current.1] = true;
        current = pq.pop().unwrap().0;
    }
    println!("{}", distance[end.0][end.1]);
}