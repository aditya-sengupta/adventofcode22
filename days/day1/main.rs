use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day01_1();
    day01_2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day01_1() {
    let mut maxcount = 0;
    let mut count = 0;
    if let Ok(lines) = read_lines("./day1") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    if count > maxcount {
                        maxcount = count;
                    }
                    count = 0;
                } else {
                    count += ip.parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("{}", maxcount)
}

fn day01_2() {
    let mut maxcounts = [0, 0, 0];
    let mut count = 0;
    if let Ok(lines) = read_lines("../data/day1") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    for j in 0..3 {
                        if count > maxcounts[j] {
                            maxcounts[j] = count;
                            maxcounts.sort();
                            break;
                        }
                    }
                    count = 0;
                } else {
                    count += ip.parse::<i32>().unwrap();
                }
            }
        }
    }
    println!("{}", maxcounts.iter().sum::<i32>());
}