use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Rope {
    head_x : i32,
    head_y : i32,
    tail_x : i32,
    tail_y : i32
}

impl Rope {
    fn is_adjacent(&self) -> bool {
        return ((self.head_x - self.tail_x).abs() <= 1) && ((self.head_y - self.tail_y).abs() <= 1)
    }

    fn move_head(&mut self, direction: &str) {
        match direction {
            "R" => self.head_x += 1,
            "U" => self.head_y += 1,
            "L" => self.head_x -= 1,
            "D" => self.head_y -= 1,
            _ => ()
        }
        self.move_tail();
    }

    fn move_tail(&mut self) {
        if !self.is_adjacent() {
            // only move the tail if we don't have adjacency
            self.tail_x += (self.head_x - self.tail_x).signum();
            self.tail_y += (self.head_y - self.tail_y).signum();
        }
    }
}

fn day9_01() {
    let mut rope = Rope{
        head_x: 0, 
        head_y: 0, 
        tail_x: 0, 
        tail_y: 0
    };
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    if let Ok(lines) = read_lines("./day9") {
        for line in lines { 
            if let Ok(ip) = line {
                let w: Vec<&str> = ip.split_whitespace().collect();
                let n: i32 = w[1].parse::<i32>().unwrap();
                for _ in 0..n {
                    rope.move_head(&w[0]);
                    tail_positions.insert((rope.tail_x, rope.tail_y));
                }
            }
        }
    }
    println!("{:?}", tail_positions.len());
}

fn day9_02() {
    let mut ropes: Vec<Rope> = vec![];
    for _ in 0..9 {
        ropes.push(Rope {
            head_x: 0,
            head_y: 0,
            tail_x: 0,
            tail_y: 0
        })
    }
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    if let Ok(lines) = read_lines("./day9") {
        for line in lines { 
            if let Ok(ip) = line {
                let w: Vec<&str> = ip.split_whitespace().collect();
                let n: i32 = w[1].parse::<i32>().unwrap();
                for _ in 0..n {
                    ropes[0].move_head(&w[0]);
                    for i in 1..9 {
                        ropes[i].head_x = ropes[i-1].tail_x;
                        ropes[i].head_y = ropes[i-1].tail_y;
                        ropes[i].move_tail();
                    }
                    tail_positions.insert((ropes[8].tail_x, ropes[8].tail_y));
                }
            }
        }
    }
    println!("{}", tail_positions.len())
}

fn main() {
    day9_01();
    day9_02();
}