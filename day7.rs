use std::fs;
// use std::cmp;

fn main() {

    let input = fs::read_to_string("day7.input")
        .expect("Something went wrong reading the file");

    let mut input = input.lines().collect::<Vec<&str>>();
    input.sort();
    println!("{:?}", input);
    let mut res: Vec<char> = Vec::new();

    for l in input {
        let first: char = l[5..6].parse::<char>().unwrap();
        let second: char = l[36..37].parse::<char>().unwrap();

        let mut index = 27;

        for i in 0..res.len() {
            if res[i] == first {
                index = i + 1;
                break;
            }
        }

        if index == 27 {
            res.push(first);
            index = res.len() - 1;
        }

        let mut i = 0;
        for i in index..res.len() {
            if res[i] > second {
                res.insert(i, second);
                break;
            } else if res[i] == second {
                break;
            }
        }
        if i == res.len() {
            res.push(second);
        }
    }

    println!("{:?}", res);

}
