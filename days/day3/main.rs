use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
extern crate itertools;
use itertools::Itertools;

fn main() {
    day03_1();
    day03_2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn char_to_priority(c: u8) -> u8 {
    if c.is_ascii_lowercase() {
        return (c as u8) - 96;
    } else {
        return (c as u8) - 64 + 26;
    }
}

fn day03_1() {
    let mut count = 0;
    if let Ok(lines) = read_lines("./day3") {
        for line in lines {
            if let Ok(ip) = line {
                let halflen = ip.len() / 2;
                let mut left_set: HashSet<u8> = HashSet::with_capacity(halflen);
                let mut right_set: HashSet<u8> = HashSet::with_capacity(halflen);
                let left_half = &ip[0..halflen].as_bytes();
                let right_half = &ip[halflen..].as_bytes();
                for j in 0..halflen {
                    left_set.insert(char_to_priority(left_half[j]));
                    right_set.insert(char_to_priority(right_half[j]));
                } 
                let overlap = left_set.intersection(&right_set).collect::<Vec<&u8>>();
                let i = overlap.into_iter().sum::<u8>();
                count += i as i32;
            }
        }
    }
    println!("{}", count);
}

fn day03_2() {
    let mut count = 0;
    let n = 3;
    if let Ok(lines) = read_lines("./day3") {
        for lines in &lines.chunks(n) {
            let mut v = Vec::<HashSet<u8>>::new();
            for _ in 0..3 { v.push(HashSet::<u8>::new()); }
            for (i, line) in lines.enumerate() {
                if let Ok(ip) = line {
                    let ipb = ip.as_bytes();
                    for j in 0..ipb.len() {
                        v[i].insert(char_to_priority(ipb[j]));
                    }
                }
            }
            let (intersection, others) = v.split_at_mut(1);
            let intersection = &mut intersection[0];
            for other in others {
                intersection.retain(|e| other.contains(e));
            }
            assert!(intersection.len() == 1);
            let k = intersection.iter().sum::<u8>();
            count += k as i32;
        }
    }
    println!("{}", count);
}