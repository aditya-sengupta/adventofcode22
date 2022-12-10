use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day8_01();
}

fn day8_01() {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    let mut nrow = 0;
    let mut ncol = 0;
    if let Ok(lines) = read_lines("./day8_test") {
        for line in lines {
            heights.push(vec![]);
            if let Ok(ip) = line {
                for b in ip.as_bytes().iter() {
                    heights[nrow].push(*b as i32 - 48);
                }
                if nrow == 0 {
                    ncol = ip.len()
                }
                nrow += 1;
            }
        }
    }

    let mut nvisible = 0;
    for i in 1..nrow-1 {
        for j in 1..ncol-1 {
            let mut visible = false;
            let mut along_path = true;
            let mut prev_value = -1;
            for k in 0..i+1 {
                along_path = along_path && (prev_value < heights[k][j]);
                prev_value = heights[k][j];
            }
            visible = visible || along_path;
            prev_value = -1;
            for k in (i..nrow).rev() {
                along_path = along_path && (prev_value < heights[k][j]);
                prev_value = heights[k][j];
            }
            visible = visible || along_path;
            prev_value = -1;
            for k in 0..j+1 {
                along_path = along_path && (prev_value < heights[i][k]);
                prev_value = heights[i][k];
            }
            visible = visible || along_path;
            prev_value = -1;
            for k in (j..ncol).rev() {
                along_path = along_path && (prev_value < heights[i][k]);
                prev_value = heights[i][k];
            }
            if visible { nvisible += 1 }
        }
    }
    println!("{} trees visible", nvisible);
}