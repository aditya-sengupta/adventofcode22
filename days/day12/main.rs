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
    day12(true);
    day12(false);
}

fn neighbors(grid: &Vec<Vec<u8>>, vis: &Vec<Vec<bool>>, current: (usize, usize), xm: usize, ym: usize, up: bool) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if current.0 != 0 { neighbors.push((current.0 - 1, current.1)) }
    if current.0 != ym - 1 { neighbors.push((current.0 + 1, current.1)) }
    if current.1 != 0 { neighbors.push((current.0, current.1 - 1)) }
    if current.1 != xm - 1 { neighbors.push((current.0, current.1 + 1)) }
    let c = grid[current.0][current.1] as u8;
    if up {
        neighbors.retain(|n| c > grid[n.0][n.1] || grid[n.0][n.1] - c <= 1);
    } else {
        neighbors.retain(|n| c < grid[n.0][n.1] || c - grid[n.0][n.1] <= 1);
    }
    neighbors.retain(|n| !vis[n.0][n.1]);
    neighbors
}

fn terminate(grid: &Vec<Vec<u8>>, current: (usize, usize), end: (usize, usize), up: bool) -> bool {
    if up {
        return current == end
    } else {
        return grid[current.0][current.1] == 97 // a
    }
}

fn day12(up: bool) {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut end = (0, 0);
    let mut pq: PriorityQueue<(usize, usize), i32> = PriorityQueue::new();
    let s = 'S' as u8;
    let e = 'E' as u8;
    let mut start = (0, 0);
    if let Ok(lines) = read_lines("./day12") {
        let mut i = 0;
        for line in lines { 
            if let Ok(ip) = line {
                grid.push(vec![]);
                for (j, b) in ip.as_bytes().iter().enumerate() {
                    let c: u8 = *b;
                    if c == s { 
                        if up { start = (i, j) }
                        grid[i].push('a' as u8);
                    } else if c == e { 
                        if up { end = (i, j) } else { start = (i, j) } 
                        grid[i].push('z' as u8)
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
    let mut distance: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    distance[start.0][start.1] = 0;
    let mut current = start;
    let mut currdist = distance[current.0][current.1] as i32;
    while !terminate(&grid, current, end, up) {
        let neighbors = neighbors(&grid, &visited, current, xm, ym, up);
        for n in neighbors {
            let d = distance[n.0][n.1].min(currdist + 1);
            distance[n.0][n.1] = d;
            pq.push(n, -d);
        }
        visited[current.0][current.1] = true;
        match pq.pop() {
            Some(p) => current = p.0,
            None => ()
        }
        currdist = distance[current.0][current.1] as i32;
    }
    println!("{}", currdist);
}