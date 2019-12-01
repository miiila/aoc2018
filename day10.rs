use std::fs;
use std::collections::HashMap;
extern crate bmp;
use bmp::Image;
use bmp::Pixel;

fn main() {

    let input = fs::read_to_string("day10.input")
        .expect("Something went wrong reading the file");
    let input = input.lines().collect::<Vec<&str>>();
    let mut coords: Vec<(i32,i32)> = Vec::new();
    let mut velocity: Vec<(i32,i32)> = Vec::new();

    for l in input {
        let coord = l[10..24].split(", ").collect::<Vec<&str>>();
        let x = coord.get(0).unwrap().trim().parse::<i32>().unwrap();
        let y = coord.get(1).unwrap().trim().parse::<i32>().unwrap();
        coords.push((x, y));
        let vel = l[36..42].split(", ").collect::<Vec<&str>>();
        velocity.push((vel.get(0).unwrap().trim().parse::<i32>().unwrap(), vel.get(1).unwrap().trim().parse::<i32>().unwrap()));
    }

    let max = (512, 512);
    let mut i = 1;
    let mut saved = 0;
    loop {
        let mut x_similarities = HashMap::new();
        for j in 0..coords.len() {
            coords[j].0 += velocity[j].0;
            coords[j].1 += velocity[j].1;
            let ys = x_similarities.entry(coords[j].0).or_insert(Vec::new());
            ys.push(coords[j].1);
        }
        let mut save = false;
        for (_x, mut ys) in x_similarities {
            ys.sort();
            if ys.len() > 6 && (*ys.last().unwrap() - ys[0] <= 8) {
                save = true;
                break;
            }
        }

        if save {
            let mut img = Image::new(max.0 as u32, max.1 as u32);
            if save {
                for (x,y) in &coords {
                    img.set_pixel(*x as u32, *y as u32, Pixel::new(255, 255, 255));
                }
                let _ = img.save(format!("./{}_second.bmp", i));
                saved += 1;
            }
        }
        if saved == 3 {
            break;
        }
        i += 1;
    }
}
