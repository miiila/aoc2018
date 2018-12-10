use std::fs;

fn main() {

    let input = fs::read_to_string("day8.input")
        .expect("Something went wrong reading the file");
    let mut input_1 = input.split_whitespace().collect();

    let sum_meta = count_meta_rec(&mut input_1);
    println!("{:?}", sum_meta);

    let mut input_2 = input.split_whitespace().collect();
    let sum_meta_2 = count_meta_rec_2(&mut input_2);
    println!("{:?}", sum_meta_2);
}

fn count_meta_rec(mut input: &mut Vec<&str>) -> i32 {
    let mut sum = 0;
    let childs = input.remove(0).parse::<i32>().unwrap();
    let metas = input.remove(0).parse::<i32>().unwrap();
    for i in 0..childs {
        sum += count_meta_rec(&mut input);
    }
    for i in 0..metas {
        sum += input.remove(0).parse::<i32>().unwrap();
    }

    return sum;
}

fn count_meta_rec_2(mut input: &mut Vec<&str>) -> i32 {
    let child_count = input.remove(0).parse::<i32>().unwrap();
    let metas = input.remove(0).parse::<i32>().unwrap();
    let mut childs = Vec::new();
    let mut sum = 0;
    for _i in 0..child_count {
        childs.push(count_meta_rec_2(&mut input));
    }
    for _i in 0..metas {
        let j = input.remove(0).parse::<i32>().unwrap();
        if child_count == 0 {
            sum += j;
        } else {
            sum += childs.get((j-1) as usize).unwrap_or(&0);
        }
    }

    return sum;
}
