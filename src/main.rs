use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day15_01("day15_test");
}

fn day15_01(fname: &str) {
    const ROWNUM: i32 = 10;
    let mut positions: Vec<Vec<i32>> = vec![];
    let mut pos: Vec<i32> = vec![];
    let mut beacon_x_on_row: HashSet<i32> = HashSet::new();
    if let Ok(lines) = read_lines(fname) {
        for line in lines { 
            if let Ok(ip) = line {
                pos.clear();
                for s in ip.split_whitespace() {
                    if s.contains("=") {
                        pos.push(s[2..s.len()-1].parse::<i32>().unwrap());
                    }
                }
                if pos[3] == ROWNUM {
                    beacon_x_on_row.insert(pos[2]);
                }
                positions.push(pos.clone())
            }
        }
    }
    let mut intervals: Vec<(i32, i32)> = vec![];
    let mut ruled_out_x: HashSet<i32> = HashSet::new();
    for p in positions {
        let distance = (p[0] - p[2]).abs() + (p[1] - p[3]).abs();
        let to_row = (p[1] - ROWNUM).abs();
        if to_row <= distance {
            let slack = distance - to_row;
            intervals.push((p[0] - slack, p[0] + slack));
            for i in -slack..slack+1 {
                ruled_out_x.insert(p[0] + i);
            }
        }
    }
    println!("{:?}", intervals);
    println!("{}", ruled_out_x.difference(&beacon_x_on_row).count());
}