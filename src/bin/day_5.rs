use std::fs;

fn main() {
    let file_path = "/home/golf0ned/code/projects/advent-of-code-25/src/input/day5.in";
    let input: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .collect();

    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(input.clone()));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut finished_ranges = false;

    let mut count = 0;
    let mut ranges = vec![];
    for line in input {
        if line == "" {
            finished_ranges = true;
            continue;
        }

        if !finished_ranges {
            let line_vec: Vec<_> = line.split("-").collect();
            ranges.push((
                line_vec[0].parse::<i64>().unwrap(),
                line_vec[1].parse::<i64>().unwrap(),
            ));
        } else {
            let id = line.parse::<i64>().unwrap();

            for (low, high) in &ranges {
                if id >= *low && id <= *high {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn part_two(input: Vec<String>) -> i64 {
    let mut ranges = vec![];
    for line in input {
        if line == "" {
            break;
        }

        let line_vec: Vec<_> = line.split("-").collect();
        ranges.push((
            line_vec[0].parse::<i64>().unwrap(),
            line_vec[1].parse::<i64>().unwrap(),
        ));
    }

    ranges.sort();

    let mut merged = vec![ranges[0]];
    for (low, high) in ranges {
        let last_high = merged.last().unwrap().1;
        if low <= last_high {
            merged.last_mut().unwrap().1 = std::cmp::max(last_high, high);
        } else {
            merged.push((low, high));
        }
    }

    merged.iter().fold(0, |sum, (l, h)| sum + h - l + 1)
}
