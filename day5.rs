use std::fs;
use std::cmp;
use std::usize;

fn main() {

    let contents = fs::read_to_string("day5.input")
        .expect("Something went wrong reading the file");

    let mut polymer = Vec::new();
    for c in contents.trim().chars() {
        polymer.push(c);
    }
    println!("{:?}", reduce(&mut polymer));
    let mut min = usize::MAX;
    for b in String::from("abcdefghijklmnopqrstuvwxyz").chars() {
        let mut polymer = Vec::new();

        let b = b.to_ascii_uppercase();
        for c in contents.trim().chars() {
            if c.to_ascii_uppercase() != b {
                polymer.push(c);
            }
        }
        min = cmp::min(min, reduce(&mut polymer));
    }

    println!("{:?}", min);
}

fn reduce(polymer: &mut Vec<char>) -> usize {
    let mut i = 0;
    while i < (polymer.len() - 1) {
        if compare(polymer[i], polymer[i+1]) {
            polymer.remove(i);
            polymer.remove(i);
            i = cmp::max(i as i32 - 2,0) as usize;
        } else {
            i += 1;
        }
    }

    polymer.len()
}

fn compare(a: char, b: char) -> bool {
    if (a.is_uppercase() && b.is_uppercase()) ||
        (a.is_lowercase() && b.is_lowercase()) {
        return false;
    }

    a.to_ascii_uppercase() == b.to_ascii_uppercase()
}
