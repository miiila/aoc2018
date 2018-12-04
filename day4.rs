use std::fs;
use std::collections::HashMap;

fn main() {
    
    let contents = fs::read_to_string("day4.input")
        .expect("Something went wrong reading the file");

	let mut contents: Vec<_> = contents.lines().collect();
	contents.sort();

	let mut slept_total: HashMap<&str,i32> = HashMap::new();
	let mut slept_minutes: HashMap<&str,[i32;60]> = HashMap::new();

	let mut cur_id = "";
	let mut minute_asleep = 0;
    let mut max: (&str,i32) = ("",0);
	for row in contents {
		let row: Vec<_> = row.split(']').collect();
		let parsed_minute = row[0].split_whitespace().collect::<Vec<&str>>()[1].split(':').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
		if row[1].contains('#') {
			cur_id  = row[1].split_whitespace().collect::<Vec<&str>>()[1];
		} else if row[1] == " falls asleep" {
			minute_asleep = parsed_minute;
		} else {
			let count = slept_total.entry(cur_id).or_insert(0);
			*count += parsed_minute - minute_asleep;
            if *count > max.1 {
                max = (cur_id, *count);
            }
			for i in minute_asleep..parsed_minute {
				let min = slept_minutes.entry(cur_id).or_insert([0;60]);
				min[i as usize] += 1;
			}
		}
	}

    // Part 1
    let mut max_min_count = 0;
    let mut max_min = 0;
	for minutes in slept_minutes.get(max.0) {
		for j in 0..minutes.len() {
            if minutes[j] > max_min_count {
                max_min_count = minutes[j];
                max_min = j as i32;
            }
		}
	}

    println!("{:#?}", max.0.trim_start_matches('#').parse::<i32>().unwrap() * max_min);

    // Part 2
    let mut max_id = 0;
    let mut max_min = 0;
    let mut max_min_count = 0;
    for (id, minutes) in slept_minutes {
        for j in 0..minutes.len() {
            if minutes[j] > max_min_count {
                max_min_count = minutes[j];
                max_min = j as i32;
                max_id = id.trim_start_matches('#').parse::<i32>().unwrap();
            }
        }
    }

    println!("{:#?}", max_id * max_min);
}
