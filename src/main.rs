// unbelievably terrible solutions, I'm so sorry

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_positions(fname: &str) -> Vec<Vec<i32>> {
    let mut positions: Vec<Vec<i32>> = vec![];
    let mut pos: Vec<i32> = vec![];
    if let Ok(lines) = read_lines(fname) {
        for line in lines { 
            if let Ok(ip) = line {
                pos.clear();
                let mut j = 0;
                for s in ip.split_whitespace() {
                    if s.contains("=") {
                        j += 1;
                        if j == 4 {
                            pos.push(s[2..s.len()].parse::<i32>().unwrap());
                        } else {
                            pos.push(s[2..s.len()-1].parse::<i32>().unwrap());
                        }
                    }
                }
                positions.push(pos.clone())
            }
        }
    }
    return positions;
}

fn main() {
    // day15_1("day15", 2000000);
    day15_2(4000000);
    // day15_2("day15_test", 20);
}

fn merge(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // https://stackoverflow.com/questions/53371736/quick-algorithm-to-create-union-of-multiple-intervals-data
    let mut merged = vec![];
    let mut start_candidate = intervals[0].0;
    let mut stop_candidate = intervals[0].1;
    for i in 1..intervals.len() {
        if intervals[i].0 <= stop_candidate {
            stop_candidate = stop_candidate.max(intervals[i].1)
        } else {
            merged.push((start_candidate, stop_candidate));
            start_candidate = intervals[i].0;
            stop_candidate = intervals[i].1;
        }
    }
    merged.push((start_candidate, stop_candidate));
    merged
}

fn day15_1(fname: &str, rownum: i32) -> Vec<(i32, i32)> {
    let positions = get_positions(fname);
    let mut intervals: Vec<(i32, i32)> = vec![];
    for p in positions {
        let r = (p[0] - p[2]).abs() + (p[1] - p[3]).abs();
        let d = (p[1] - rownum).abs();
        if r >= d {
            let slack = r - d;
            intervals.push( (p[0] - slack, p[0] + slack) );
        }
    }
    intervals.sort_by(|v, w| v.0.cmp(&w.0));
    return merge(intervals);
    /* 
    let mut s: i32 = 0;
    for interval in intervals {
        s += interval.1 - interval.0;
    }
    println!("{}", s);
    */
}

fn day15_2(max_coord: i32) {
    let mut children = vec![];
    let n = 20;
    for nth in 0..n {
        children.push(thread::spawn(move || {
            for y in (max_coord * nth / n)..(max_coord * (nth + 1) / n) {
                let mut intervals = day15_1("day15", y);
                for i in 0..intervals.len() {
                    intervals[i].0 = intervals[i].0.max(0);
                    intervals[i].1 = intervals[i].1.min(max_coord);
                }
                let mut x = intervals[0].1;
                for i in 1..intervals.len() {
                    if intervals[i].0 - x > 1 {
                        println!("{}, {}", x + 1, y);
                        break;
                    }
                    x = intervals[i].1;
                }
            }
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

}