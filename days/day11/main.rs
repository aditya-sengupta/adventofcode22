// this is a working? solution to part b

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Monkey {
    items: Vec<i64>,
    binaryop: String,
    rhs: i64,
    divis: i64,
    true_target: i64,
    false_target: i64,
    ninspect: i64
}

impl Monkey {
    fn operation(&self, worry: i64) -> i64 {
        let r: i64;
        if self.rhs == 0 {
            r = worry;
        } else {
            r = self.rhs
        }
        match self.binaryop.as_str() {
            "+" => worry + r,
            "*" => worry * r,
            _ => 0
        }
    }

    fn target(&self, worry: i64) -> i64 {
        if worry.checked_rem(self.divis).unwrap() == 0 {
            return self.true_target
        } else {
            return self.false_target
        }
    }
}

fn parse_numbers(words: Vec<&str>) -> Vec<i64> {
    let mut nums: Vec<i64> = vec![];
    for i in 2..words.len() {
        nums.push(words[i][0..2].parse::<i64>().unwrap());
    }
    return nums
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut items: Vec<i64> = vec![];
    let mut binaryop: String = "".to_string();
    let mut rhs: i64 = 0;
    let mut divis: i64 = 0;
    let mut true_target: i64 = 0;
    let mut false_target: i64;
    if let Ok(lines) = read_lines("./day11") {
        for line in lines { 
            if let Ok(ip) = line {
                let words: Vec<&str> = ip.split_whitespace().collect();
                if words.len() > 0 {
                    match words[0] {
                        "Starting" => items = parse_numbers(words),
                        "Operation:" => { 
                            binaryop = words[4].to_string();
                            rhs = match words[5].parse::<i64>() {
                                Ok(p) => p,
                                _error => 0
                            }
                        }
                        "Test:" => divis = words[3].parse::<i64>().unwrap(),
                        "If" => {
                            match words[1] {
                                "true:" => true_target = words[5].parse::<i64>().unwrap(),
                                "false:" => {
                                    false_target = words[5].parse::<i64>().unwrap();
                                    let m = Monkey {
                                        items : items.clone(),
                                        binaryop : binaryop.clone(),
                                        rhs : rhs,
                                        divis : divis,
                                        true_target : true_target,
                                        false_target : false_target,
                                        ninspect : 0
                                    };
                                    monkeys.push(m);
                                },
                                _ => ()
                            }
                        }
                        _ => ()
                    }
                }
            }
        }
    }
    monkeys
}

fn main() {
    day11(20, 3);
    day11(10000, 1);
}

fn day11(nr: i64, worrydiv: i64) {
    let mut monkeys = parse_monkeys();
    let mut f = 1;
    for m in &monkeys {
        f *= m.divis;
    }
    let nm = monkeys.len();
    for _ in 0..nr {
        for i in 0..nm {
            for item in monkeys[i].items.clone() {
                let w = (monkeys[i].operation(item) / worrydiv) % f;
                let target = monkeys[i].target(w) as usize;
                monkeys[target].items.push(w);
                monkeys[i].ninspect += 1;
            }
            monkeys[i].items = vec![];
        }
    }

    let mut business: Vec<i64> = Vec::new();
    for m in monkeys {
        business.push(m.ninspect);
    }
    business.sort();
    println!("{}", business[nm - 1] * business[nm - 2]);
}