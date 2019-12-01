use std::collections::HashMap;

fn main() {

    let serial = 5235;
    let mut grid: [[i32;300];300] = [[0;300];300];
    let mut max_power = 0;
    let mut max_cell = (0,0,0);
    let mut max_three = 0;
    let mut max_cell_three = (0,0);

    let mut powers = HashMap::new();
    for x in (0..300).rev() {
        for y in (0..300).rev() {
            let mut v: Vec<i32> = Vec::new();
            let power = (((((x+10) * y + serial) * (x+10)) / 100) % 10) - 5;
            grid[x as usize][y as usize] = power;
            v.push(grid[x as usize][y as usize]);
            let mut siblings_sum = 0;
            let mut i = 1;
            while x+i < 300 && y+i < 300 {
                siblings_sum += grid[(x+i) as usize][y as usize];
                siblings_sum += grid[(x) as usize][(y+i) as usize];
                let p: &Vec<i32> = powers.get(&(x+1,y+1)).unwrap();
                let power_sum_diagonal = grid[x as usize][y as usize] + siblings_sum + p[(i-1) as usize];
                v.push(power_sum_diagonal);
                if i == 2 && power_sum_diagonal > max_three {
                    max_three = power_sum_diagonal;
                    max_cell_three = (x,y);
                }
                if power_sum_diagonal > max_power {
                    max_power = power_sum_diagonal;
                    max_cell = (x,y,i+1);
                }
            i += 1;
            }
            powers.insert((x,y), v);
        }
    }

    println!("{:?} {:?}", max_three, max_cell_three);
    println!("{:?} {:?}", max_power, max_cell);

}
