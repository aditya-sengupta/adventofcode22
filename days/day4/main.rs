use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day04_1();
    day04_2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day04_1() {
    let mut count = 0;
    if let Ok(lines) = read_lines("./day4") {
        for line in lines {
            if let Ok(ip) = line {
                let ranges: Vec<&str> = ip.split(',').collect();
                let range0: Vec<&str> = ranges[0].split('-').collect();
                let range1: Vec<&str> = ranges[1].split('-').collect();
                let lower0 = range0[0].parse::<i32>().ok();
                let upper0 = range0[1].parse::<i32>().ok();
                let lower1 = range1[0].parse::<i32>().ok();
                let upper1 = range1[1].parse::<i32>().ok();
                if ((lower0 <= lower1) && (upper0 >= upper1)) || ((lower1 <= lower0) && (upper1 >= upper0)) { 
                    count += 1 
                }
            }
        }
    }
    println!("{}", count);
}

fn day04_2() {
    let mut count = 0;
    if let Ok(lines) = read_lines("./day4") {
        for line in lines {
            if let Ok(ip) = line {
                let ranges: Vec<&str> = ip.split(',').collect();
                let range0: Vec<&str> = ranges[0].split('-').collect();
                let range1: Vec<&str> = ranges[1].split('-').collect();
                let lower0 = range0[0].parse::<i32>().ok();
                let upper0 = range0[1].parse::<i32>().ok();
                let lower1 = range1[0].parse::<i32>().ok();
                let upper1 = range1[1].parse::<i32>().ok();
                if !(upper0 < lower1) && !(upper1 < lower0) { 
                    count += 1 
                }
            }
        }
    }
    println!("{}", count);
}
