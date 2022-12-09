use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct DirectoryTree {
    sizes: HashMap<String, i32>,
    parents: HashMap<String, String>,
    current: String
}

impl DirectoryTree {
    fn dotdot(&mut self) {
        let pdir = self.parents.get(&self.current);
        self.current = pdir.unwrap().to_string();
    }

    fn new_child(&mut self, word: String) {
        self.parents.insert(word.clone(), self.current.to_string());
        self.current = word;
        self.sizes.insert(self.current.to_string(), 0);
    }

    fn add_size(&mut self, fsize: i32) {
        let current_size = self.sizes.get(&self.current).unwrap();
        self.sizes.insert(self.current.to_string(), current_size + fsize);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_words(ip: String) -> Vec<String> {
    let fwords: Vec<&str> = ip.split_whitespace().collect::<Vec<&str>>();
    let mut words: Vec<String> = vec![];
    for w in fwords {
        words.push(w.to_string())
    } // I hate this
    return words
}

fn main() {
    day7_01();
}

fn day7_01() {
    let mut tree = DirectoryTree {
        sizes: HashMap::new(),
        parents: HashMap::new(),
        current: "".to_string()
    };
    if let Ok(lines) = read_lines("./day7_test") {
        for line in lines {
            if let Ok(ip) = line {
                let words = parse_words(ip);
                if words[0] == "$" && words[1] == "cd" {
                    if words[2] == ".." { 
                        // move up one level
                        tree.dotdot();
                    } else {
                        // (presumably) new directory found
                        tree.new_child(words[2].to_string());
                    }
                } else if words[0].parse::<i32>().is_ok() {
                    // file name
                    let fsize = words[0].parse::<i32>().unwrap();
                    println!("{}", fsize);
                    tree.add_size(fsize);
                }
            }
        }
    }
    println!("{:?}", tree.sizes);
    println!("{:?}", tree.parents);
}