use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use itertools::zip;

struct DirectoryTree {
    sizes: HashMap<String, i32>,
    parents: HashMap<String, String>,
    nchildren: HashMap<String, i32>,
    current: String
}

impl DirectoryTree {
    fn dotdot(&mut self) {
        let pdir = self.parents.get(&self.current);
        self.current = pdir.unwrap().to_string();
    }

    fn new_child(&mut self, mut word: String) {
        while self.parents.contains_key(&word) {
            word.push_str("_"); // oh this sucks
        }
        self.parents.insert(word.clone(), self.current.to_string());
        let nch = match self.nchildren.get(&self.current) {
            Some(p) => p,
            None => &0
        };
        self.nchildren.insert(self.current.to_string(), nch + 1);
        self.current = word;
        self.sizes.insert(self.current.to_string(), 0);
        self.nchildren.insert(self.current.to_string(), 0);
    }

    fn add_size(&mut self, fsize: i32) {
        let current_size = self.sizes.get(&self.current).unwrap();
        self.sizes.insert(self.current.to_string(), current_size + fsize);
    }

    fn reroot(&mut self) {
        self.parents.remove("/");
        self.nchildren.remove("");
    }
}

fn delete_size(sizes: &HashMap<String, i32>, used_size: i32) -> i32 {
    let free_space = 70000000 - used_size;
    let delete_size = (30000000 - free_space).max(0);
    let mut current_delete_size: i32 = used_size;
    for (_, &size) in sizes.iter().clone() {
        if size < current_delete_size && size >= delete_size {
            current_delete_size = size;
        }
    }
    return current_delete_size
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
    day7();
}

fn day7() {
    let mut tree = DirectoryTree {
        sizes: HashMap::new(),
        parents: HashMap::new(),
        nchildren: HashMap::new(),
        current: "".to_string()
    };
    let mut used_size = 0;

    if let Ok(lines) = read_lines("./day7") {
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
                    tree.add_size(fsize);
                    used_size += fsize;
                }
            }
        }
    }
    tree.reroot();

    while tree.parents.len() > 0 {
        let mut dirs_to_remove: Vec<String> = vec![];
        let mut parents_to_remove: Vec<String> = vec![];
        for (dir, parent) in tree.parents.clone() {
            let nch = match tree.nchildren.get(&dir) {
                Some(p) => p,
                None => &0
            };
            if nch == &0 {
                // this directory is a leaf, 
                // so we can safely add its size to its parent
                // and remove the dir -> parent map so we don't do it again
                let parent_size = tree.sizes.get(&parent).unwrap();
                let child_size = tree.sizes.get(&dir).unwrap();
                tree.sizes.insert(parent.clone(), parent_size + child_size);
                if parent != "/" {
                    // keep track of how many unresolved children the parent has
                    // so we don't prematurely add its size to the grandparent
                    let parent_nch = match tree.nchildren.get(&parent) {
                        Some(p) => p,
                        None => &0
                    };
                    tree.nchildren.insert(parent.clone(), parent_nch - 1);
                }
                dirs_to_remove.push(dir.clone());
                parents_to_remove.push(parent.clone());
            }
        }
        for (dir, parent) in zip(dirs_to_remove, parents_to_remove) {
            tree.parents.remove(&dir);
            tree.nchildren.remove(&parent);
        }
    }
    let mut total_size = 0;
    for (_, size) in tree.sizes.iter().clone() {
        if size <= &100000 { total_size += size }
    }
    println!("Total size {}", total_size);
    println!("Min size to delete {}", delete_size(&tree.sizes, used_size));
}