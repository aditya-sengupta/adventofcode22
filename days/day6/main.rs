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
    day6(4);
    day6(14);
}

fn day6(n: usize) {
    if let Ok(lines) = read_lines("./day6") {
        for line in lines {
            if let Ok(ip) = line {
                let mut last_n: Vec<u8> = Vec::new();
                let mut i = 0;
                for b in ip.as_bytes() {
                    if i < n { 
                        last_n.push(*b);
                    } else {
                        let last_set: HashSet<&u8> = HashSet::from_iter(&last_n);
                        if last_set.len() == n { 
                            println!("{}", i); 
                            break;
                        }
                        last_n[i % n] = *b;
                    }
                    i += 1;
                }
            }
        }
    }
}