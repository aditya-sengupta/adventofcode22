use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day10_01();
    day10_02();
}

struct CPU {
    cycle: i32,
    x: i32,
    total_strength: i32
}

impl CPU {
    fn watch(&mut self) {
        if self.cycle.checked_rem(40).unwrap() == 20 && self.cycle <= 220 {
            // println!("at cycle {}, strength {}", self.cycle, self.cycle * self.x);
            self.total_strength += self.cycle * self.x;
        }
    }

    fn draw(&self, screen: &mut Vec<bool>) {
        if (self.x - (self.cycle - 1).checked_rem(40).unwrap()).abs() < 2 {
            screen[self.cycle as usize - 1] = true;
        }
    }
}

fn day10_01() {
    let mut cpu = CPU { cycle : 0, x : 1, total_strength : 0 };
    if let Ok(lines) = read_lines("./day10") {
        for line in lines { 
            if let Ok(ip) = line {
                let words: Vec<&str> = ip.split_whitespace().collect();
                if words[0] == "addx" {
                    let v = words[1].parse::<i32>().unwrap();
                    cpu.cycle += 1;
                    cpu.watch();
                    cpu.cycle += 1;
                    cpu.watch();
                    cpu.x += v;
                } else if words[0] == "noop" {
                    cpu.cycle += 1;
                    cpu.watch();
                }
            }
        }
    }
    println!("{}", cpu.total_strength);
}

fn print_screen(screen: Vec<bool>) {
    for i in 0..240 {
        if screen[i] {
            print!("#");
        } else {
            print!(".");
        }
        if i.checked_rem(40).unwrap() == 39 {
            println!();
        }
    }
    println!();
}

fn day10_02() {
    let mut cpu = CPU { cycle : 0, x : 1, total_strength : 0 };
    let mut screen: Vec<bool> = iter::repeat(false).take(240).collect();
    if let Ok(lines) = read_lines("./day10") {
        for line in lines { 
            if let Ok(ip) = line {
                let words: Vec<&str> = ip.split_whitespace().collect();
                if words[0] == "addx" {
                    let v = words[1].parse::<i32>().unwrap();
                    cpu.cycle += 1;
                    cpu.draw(&mut screen);
                    cpu.cycle += 1;
                    cpu.draw(&mut screen);
                    cpu.x += v;
                } else if words[0] == "noop" {
                    cpu.cycle += 1;
                    cpu.draw(&mut screen);
                }
            }
        }
    }
    print_screen(screen);
}