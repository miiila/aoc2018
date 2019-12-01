use std::fs;
fn main() {

    let mut patterns: [bool;32] = [false;32];

    let input = fs::read_to_string("day12.input")
        .expect("Something went wrong reading the file");

    for l in input.lines() {
        let index = convert(&String::from(l.split(" => ").nth(0).unwrap()));
        patterns[index as usize] = *l.split(" => ").nth(1).unwrap() == *"#";
    }

    let mut current: Vec<bool> = Vec::new();
    let input = "##.######...#.##.#...#...##.####..###.#.##.#.##...##..#...##.#..##....##...........#.#.#..###.#";
    for c in input.chars() {
        let v = if c == '#' {
            true
        } else {
            false
        };
        current.push(v);
    }

    let mut start = 0;

    // println!("{:?} {:?}", current, start);
    let mut next: Vec<bool>;
    let mut prev_res = 0;
    let mut diff = 0;
    let mut gen: i64 = 0;
    let mut res;
    loop {
        if current[0..5].contains(&true) {
            for _i in 0..4 {
                current.insert(0, false);
            }
            start += 4;
        }
        if current[current.len()-4..].contains(&true) {
            for _i in 0..4 {
                current.push(false);
            }
        }
        next = vec![false, false];
        for i in 0..(current.len() - 4) {
            let num = convert_from_bool(&current[i..i+5]);
            next.push(patterns[num as usize]);
        }
        current = Vec::new();
        for i in next {
            current.push(i);
        }
        gen += 1;
        res = 0;
        for (i,v) in current.iter().enumerate() {
            if *v {
                // println!("{:?} {:?}",v, (i as i32) - start);
                res += (i as i32) - start;
            }
        }
        if res - prev_res == diff {
            println!("{:?} {:?} {:?} {:?}", prev_res, res, diff, gen);
            break;
        }
        diff = res - prev_res;
        prev_res = res;
    }

    println!("{:?}", (50000000000-gen) * diff as i64 + res as i64);
}

fn convert (pot: &str) -> u8 {
    let mut num: u8 = 0;
    for (i,c) in pot.chars().rev().enumerate() {
        if c == '#' {
            num |= 2_u8.pow(i as u32)
        }
    }
    return num;
}

fn convert_from_bool (pot: &[bool]) -> u8 {
    let mut num: u8 = 0;
    for (i,c) in pot.iter().rev().enumerate() {
        if *c {
            num |= 2_u8.pow(i as u32)
        }
    }
    return num;
}
