fn main() {

    let mut scores = vec![3,7];
    let limit = 633601;
    let mut elf1 = 0;
    let mut elf2 = 1;

    // Part 1
    while scores.len() < limit + 11 {
        let new_score = scores[elf1] + scores[elf2];
        if new_score > 9 {
            scores.push(1);
        }
        scores.push(new_score % 10);
        elf1 = (elf1 + 1 + scores[elf1]) % scores.len();
        elf2 = (elf2 + 1 + scores[elf2]) % scores.len();
    }

    println!("{:?}", &scores[limit..limit+10]);

    // Part 2
    let mut scores = vec![3,7];
    let limit = [6,3,3,6,0,1];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut index = 0;
    loop {
        let new_score = scores[elf1] + scores[elf2];
        if new_score > 9 {
            scores.push(1);
        }
        scores.push(new_score % 10);
        elf1 = (elf1 + 1 + scores[elf1]) % scores.len();
        elf2 = (elf2 + 1 + scores[elf2]) % scores.len();
        if scores.len() >= limit.len() {
            if scores[index..index+limit.len()] == limit {
                println!("{:?}", index);
                break;
            } else {
                index += 1;
            }
        }
    }
}
