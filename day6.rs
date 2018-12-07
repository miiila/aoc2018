use std::fs;
use std::i32;
use std::cmp;

fn main() {

    let input = fs::read_to_string("day6.input")
        .expect("Something went wrong reading the file");

    let mut coords: [(i32,i32);50] = [(0,0);50];
    let mut res: [i32;50] = [0;50];

    let mut i = 0;
    for l in input.lines() {
        let l: Vec<_> = l.split(", ").collect();
        let k = (l[0].parse::<i32>().unwrap(),l[1].parse::<i32>().unwrap());
        coords[i] = k;
        i += 1;
    }

    // First

    let mut grid: [[(i32,i32);400]; 400] = [[(i32::MAX,-1);400]; 400];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for (k, coord) in coords.iter().enumerate() {
                let distance = (i as i32 - coord.0).abs() + (j as i32 - coord.1).abs();
                if distance < grid[i][j].0 {
                    if grid[i][j].1 != -1 {
                        res[grid[i][j].1 as usize] -= 1;
                    }
                    grid[i][j] = (distance, k as i32);
                    res[k] += 1;
                } else if distance == grid[i][j].0 && grid[i][j].1 != -1 {
                    res[grid[i][j].1 as usize] -= 1;
                    grid[i][j].1 = -1;
                }
            }
        }
    }

    for i in 0..grid.len() {
        if grid[0][i].1 != -1 {
            res[grid[0][i].1 as usize] = -1;
        }
        if grid[i][0].1 != -1 {
            res[grid[i][0].1 as usize] = -1;
        }
        if grid[i][grid.len()-1].1 != -1 {
            res[grid[i][grid.len()-1].1 as usize] = -1;
        }
        if grid[grid.len()-1][i].1 != -1 {
            res[grid[grid.len()-1][i].1 as usize] = -1;
        }
    }

    let mut max = 0;
    for i in res.iter() {
        max = cmp::max(max, *i);
    }

    println!("{:?}", max);

    // Part 2
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut finished = true;
            let mut distance = 0;
            for coord in coords.iter() {
                distance += (i as i32 - coord.0).abs() + (j as i32 - coord.1).abs();
                if distance >= 10000 {
                    finished = false;
                    break;
                }
            }
            if finished {
                res += 1;
            }
        }
    }

    println!("{:?}", res);
}
