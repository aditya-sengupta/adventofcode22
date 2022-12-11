use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    day8();
}

fn day8() {
    let mut heights: Vec<Vec<i32>> = Vec::new();
    let mut visibles: Vec<Vec<bool>> = Vec::new();
    let mut nrow = 0;
    let mut ncol = 0;
    if let Ok(lines) = read_lines("./day8") {
        for line in lines {
            heights.push(vec![]);
            visibles.push(vec![]);
            if let Ok(ip) = line {
                for b in ip.as_bytes().iter() {
                    heights[nrow].push(*b as i32 - 48);
                    visibles[nrow].push(false);
                }
                if nrow == 0 {
                    ncol = ip.len()
                }
                nrow += 1;
            }
        }
    }

    let mut h: i32;
    for i in 0..nrow {
        // the left and right paths on each row
        let mut j = 0;
        let mut prev = -1;
        while j < ncol {
            h = heights[i][j];
            if prev < h { 
                visibles[i][j] = true;
            }
            prev = prev.max(h);
            j += 1;
        }
        j = ncol - 1;
        prev = -1;
        while j > 0 {
            h = heights[i][j];
            if prev < h { 
                visibles[i][j] = true;
            }
            prev = prev.max(h);
            j -= 1;
        }
    }
    for j in 0..ncol {
        // the up and down paths on each column
        let mut i = 0;
        let mut prev = -1;
        while i < nrow {
            h = heights[i][j];
            if prev < h { 
                visibles[i][j] = true;
            }
            prev = prev.max(h);
            i += 1;
        }
        i = nrow - 1;
        prev = -1;
        while i > 0  {
            h = heights[i][j];
            if prev < h { 
                visibles[i][j] = true;
            }
            prev = prev.max(h);
            i -= 1;
        }
    }
    
    let mut nvisibles = 0;
    for i in 0..nrow { for j in 0..ncol {
        if visibles[i][j] { nvisibles += 1 }
    }}
    println!("{}", nvisibles);

    let mut scenic_scores: Vec<Vec<i32>> = Vec::new();
    for i in 0..nrow {
        scenic_scores.push(vec![]);
        for _ in 0..ncol {
            scenic_scores[i].push(1);
        }
    }

    // populate the scenic scores matrix
    for i in 0..nrow { for j in 0..ncol {
        h = heights[i][j];
        let mut view_length = 0;
        let mut k = i;
        let mut blocked = false;
        while k > 0 && !blocked {
            k -= 1;
            view_length += 1;
            if heights[k][j] >= h { blocked = true }
        }
        scenic_scores[i][j] *= view_length;
        blocked = false;
        k = i;
        view_length = 0;
        while k < nrow - 1 && !blocked {
            k += 1;
            view_length += 1;
            if heights[k][j] >= h { blocked = true }
        }
        scenic_scores[i][j] *= view_length;
        blocked = false;
        k = j;
        view_length = 0;
        while k > 0 && !blocked {
            k -= 1;
            view_length += 1;
            if heights[i][k] >= h { blocked = true }
        }
        scenic_scores[i][j] *= view_length;
        blocked = false;
        k = j;
        view_length = 0;
        while k < ncol - 1 && !blocked {
            k += 1;
            view_length += 1;
            if heights[i][k] >= h { blocked = true }
        }
        scenic_scores[i][j] *= view_length;
    }}

    let mut max_scenic = 0;
    for i in 0..nrow {
        max_scenic = max_scenic.max(*scenic_scores[i].iter().max().unwrap());
    }
    println!("{}", max_scenic)
}