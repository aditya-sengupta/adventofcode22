use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // day14_01("./day14");
    day14_02("./day14_test");
}

fn grid_size(fname: &str, floor: bool) -> (Vec<i32>, Vec<i32>) {
    let mut mins = vec![i32::MAX, 0];
    let mut maxes = vec![0, 0];
    if let Ok(lines) = read_lines(fname) {
        for line in lines { 
            if let Ok(ip) = line {
                for item in ip.split("->") {
                    for (j, num) in item.split(",").enumerate() {
                        let n = num.trim().parse::<i32>().unwrap();
                        mins[j] = mins[j].min(n);
                        maxes[j] = maxes[j].max(n);
                    }
                }
            }
        }
    }
    if floor {
        maxes[1] += 2;
    }
    return (maxes, mins)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn make_grid(fname: &str, floor: bool) -> Vec<Vec<bool>> {
    let lims: (Vec<i32>, Vec<i32>) = grid_size(fname, floor);
    let mut grid = vec![vec![false; (lims.0[1] - lims.1[1] + 1) as usize]; (lims.0[0] - lims.1[0] + 1) as usize];
    if let Ok(lines) = read_lines(fname) {
        for line in lines { 
            if let Ok(ip) = line {
                let mut last_point: [i32; 2] = [-1, -1];
                for item in ip.split("->") {
                    let mut coords = [0, 0];
                    for (j, num) in item.split(",").enumerate() {
                        let n = num.trim().parse::<i32>().unwrap();
                        coords[j] = n - lims.1[j];
                    }
                    grid[coords[0] as usize][coords[1] as usize] = true;
                    if last_point[0] == coords[0] && last_point[1] != coords[1] {
                        let l = last_point[1].min(coords[1]);
                        let u = last_point[1].max(coords[1]);
                        for y in l..u {
                            grid[coords[0] as usize][y as usize] = true;
                        }
                    } else if last_point[0] != coords[0] && last_point[1] == coords[1] {
                        let l = last_point[0].min(coords[0]);
                        let u = last_point[0].max(coords[0]);
                        for x in l..u {
                            grid[x as usize][coords[1] as usize] = true;
                        }
                    }
                    last_point = coords;
                }
            }
        }
    }
    return transpose(grid);
}

fn filled(grid: &Vec<Vec<bool>>, sand: &Vec<Vec<bool>>, coords: [usize; 2]) -> bool {
    grid[coords[0]][coords[1]] || sand[coords[0]][coords[1]]
}

fn day14_01(fname: &str) {
    let lims: (Vec<i32>, Vec<i32>) = grid_size(fname, false);
    let grid = make_grid(fname, false);
    let mut sand = grid.clone();
    let source: [usize; 2] = [0, 500 - lims.1[0] as usize];
    let mut current_sand = source;
    let mut move_possible: bool;
    let mut outflow: bool = false;
    let mut n = 0;
    let ulim = grid.len();
    let rlim = grid[0].len();
    while !outflow {
        move_possible = true;
        while move_possible {
            current_sand[0] += 1;
            if current_sand[0] == ulim {
                outflow = true;
                break;
            }
            // try down
            if filled(&grid, &sand, current_sand) {
                if current_sand[1] == 0 {
                    outflow = true;
                    break;
                }
                current_sand[1] -= 1;
                // try down-left
                if filled(&grid, &sand, current_sand) {
                    current_sand[1] += 2;
                    if current_sand[1] == rlim {
                        outflow = true;
                        break
                    }
                    // try down-right
                    if filled(&grid, &sand, current_sand) {
                        current_sand[0] -= 1;
                        current_sand[1] -= 1;
                        sand[current_sand[0]][current_sand[1]] = true;
                        current_sand = source;
                        move_possible = false;
                        n += 1;
                    }
                }
            }
        }
    }
    println!("{}", n);
    // print(&grid, &sand);
}

fn day14_02(fname: &str) {
    let grid = make_grid(fname, true);
    print(&grid, &grid);
}

fn print(grid: &Vec<Vec<bool>>, sand: &Vec<Vec<bool>>) {
    for line in 0..grid.len() {
        for p in 0..grid[line].len() {
            print!("{}", match grid[line][p] {
                true => "#",
                false => match sand[line][p] {
                    true => "o",
                    false => "."
                }
            })
        }
        println!();
    }
}