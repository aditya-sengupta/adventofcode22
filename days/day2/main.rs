// this is terrible and incredibly hardcoded

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day02_1();
    day02_2();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn day02_1() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./day2") {
        for line in lines {
            if let Ok(ip) = line {
                let player = ip.as_bytes()[2] as char;
                score += match player {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => 0
                };
                score += match ip.as_str() {
                    "A X" => 3,
                    "A Y" => 6,
                    "A Z" => 0,
                    "B X" => 0,
                    "B Y" => 3,
                    "B Z" => 6,
                    "C X" => 6,
                    "C Y" => 0,
                    "C Z" => 3,
                    _ => 0
                }
            }
        }
    }
    println!("{}", score)
}

fn day02_2() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./day2") {
        for line in lines {
            if let Ok(ip) = line {
                let player = ip.as_bytes()[2] as char;
                // now this one is for the result of the game
                score += match player {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => 0
                };
                // and this one is for which of R, P, S you got
                score += match ip.as_str() {
                    "A X" => 3,
                    "A Y" => 1,
                    "A Z" => 2,
                    "B X" => 1,
                    "B Y" => 2,
                    "B Z" => 3,
                    "C X" => 2,
                    "C Y" => 3,
                    "C Z" => 1,
                    _ => 0
                }
            }
        }
    }
    println!("{}", score)
}