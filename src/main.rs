use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day7_01();
}

fn day7_01() {
    let mut directory_sizes = HashMap::<String, i32>::new();
    let mut directory_parents = HashMap::<String, String>::new();
    let mut current_directory: String = "".to_string();
    if let Ok(lines) = read_lines("./day7_test") {
        for line in lines {
            if let Ok(ip) = line {
                let fwords: Vec<&str> = ip.split_whitespace().collect::<Vec<&str>>();//.map(|item| item.into());
                let mut words: Vec<String> = vec![];
                for w in fwords {
                    words.push(w.to_string())
                } // I hate this
                if words[0] == "$" && words[1] == "cd" {
                    if words[2] == ".." { 
                        current_directory = directory_parents.get(&current_directory).unwrap().to_string();
                    } else {
                        // (presumably) new directory found
                        directory_parents.insert(current_directory, words[2].to_string());
                        directory_sizes.insert(current_directory, -1);
                    }
                }
            }
        }
    }
    println!("{:?}", directory_parents);
}