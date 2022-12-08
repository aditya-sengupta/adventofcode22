use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day05(true);
    day05(false);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn move_boxes_1(ip: &String, inventory: &mut Vec<Vec<char>>) {
    let commands: Vec<&str> = ip.split_whitespace().collect();
    let num_to_move = commands[1].parse::<i32>().ok().unwrap();
    let from_idx = commands[3].parse::<i32>().ok().unwrap() - 1;
    let to_idx = commands[5].parse::<i32>().ok().unwrap() - 1;
    for _ in 0..num_to_move {
        let val = inventory[from_idx as usize].pop().unwrap();
        inventory[to_idx as usize].push(val);
    }
}

fn move_boxes_2(ip: &String, inventory: &mut Vec<Vec<char>>) {
    let commands: Vec<&str> = ip.split_whitespace().collect();
    let num_to_move = commands[1].parse::<i32>().ok().unwrap();
    let from_idx = commands[3].parse::<i32>().ok().unwrap() - 1;
    let to_idx = commands[5].parse::<i32>().ok().unwrap() - 1;
    let mut stack: Vec<char> = Vec::new();
    for _ in 0..num_to_move { stack.push(inventory[from_idx as usize].pop().unwrap()); }
    for _ in 0..num_to_move { inventory[to_idx as usize].push(stack.pop().unwrap()) }
}

fn day05(move_as_1: bool) {
    let mut reading_boxes = true;
    let mut is_first_line = true;
    let mut ncols = 0;
    let mut inventory: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./day5") {
        for line in lines {
            if let Ok(ip) = line {
                let ipb = ip.as_bytes();
                if is_first_line {
                    ncols = (ip.len() as i32 / 4) + 1;
                    is_first_line = false;
                    for _ in 0..ncols {
                        inventory.push(Vec::new());
                    }
                }
                if !reading_boxes {
                    if move_as_1 { move_boxes_1(&ip, &mut inventory); } else { move_boxes_2(&ip, &mut inventory); }
                }
                if ip.len() == 0 && reading_boxes {
                    reading_boxes = false;
                    for i in 0..ncols-1 {
                        inventory[i as usize].reverse();
                    }
                }
                if reading_boxes {
                    for i in 0..ncols {
                        let ii = i as usize;
                        if ipb[4*ii+1] >= 65 {
                            inventory[ii].push(ipb[4*ii+1] as char);
                        }
                    }
                }
            }
        }
    }
    for i in 0..ncols {
        let l = &inventory[i as usize];
        print!("{}", l[l.len() - 1]);
    }
    println!();
}

fn _day05_2() {
    println!("t");
}