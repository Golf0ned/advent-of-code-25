use std::fs;

fn main() {
    let file_path = "/home/golf0ned/code/projects/advent-of-code-25/src/input/day2.in";
    let input: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .trim_end()
        .split(",")
        .map(String::from)
        .collect();

    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(input.clone()));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut sum = 0;
    let ranges: Vec<_> = input
        .iter()
        .map(|s| {
            let pair: Vec<_> = s.split("-").collect();
            (
                pair[0].parse::<i64>().unwrap(),
                pair[1].parse::<i64>().unwrap(),
            )
        })
        .collect();

    for (low, high) in ranges {
        for i in low..=high {
            let num_str = i.to_string();
            let (front, back) = num_str.split_at(num_str.len() / 2);
            if front == back {
                sum += i
            }
        }
    }

    sum
}

fn part_two(input: Vec<String>) -> i64 {
    let mut sum = 0;
    let ranges: Vec<_> = input
        .iter()
        .map(|s| {
            let pair: Vec<_> = s.split("-").collect();
            (
                pair[0].parse::<i64>().unwrap(),
                pair[1].parse::<i64>().unwrap(),
            )
        })
        .collect();

    for (low, high) in ranges {
        for num in low..=high {
            let num_str = num.to_string();
            let len = num_str.len();
            for i in 1..len / 2 + 1 {
                if len % i == 0 {
                    let repeated = &num_str[0..i];
                    if num_str == repeated.repeat(len / i) {
                        sum += num;
                        break;
                    }
                }
            }
        }
    }

    sum
}
