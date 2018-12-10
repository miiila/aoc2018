use std::cmp;
use std::collections::VecDeque;

fn main() {

    let _last = 70784; // First Part
    let last = 7078400; // Second Part
    let players = 452;
    let mut scores: [i64;452] = [0;452];
    let mut current_player = players-1;

    let mut marbles: VecDeque<u32> = VecDeque::new();
    marbles.push_front(0);

    for i in 1..last+1 {
        current_player = (current_player + 1) % players;
        if i % 23 == 0 {
            for _i in 0..7 {
                let m = marbles.pop_back().unwrap();
                marbles.push_front(m);
            }
            scores[current_player] += i as i64 + marbles.pop_back().unwrap() as i64;
            let m = marbles.pop_front().unwrap();
            marbles.push_back(m);
            continue;
        }
        let m = marbles.pop_front().unwrap();
        marbles.push_back(m);
        marbles.push_back(i);
    }

    let mut max_score = 0;
    for i in 0..players {
        max_score = cmp::max(max_score, scores[i]);
    }

    println!("{:?}", max_score);
}
