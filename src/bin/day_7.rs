use std::fs;

fn main() {
    let file_path = "/home/golf0ned/code/projects/advent-of-code-25/src/input/day7.in";
    let input: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .collect();

    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(input.clone()));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut splits = 0;
    let mut prev: Vec<_> = vec![false; input[0].len()];

    for row in input {
        let mut cur = prev.to_vec();

        for (i, c) in row.chars().enumerate() {
            match c {
                'S' => cur[i] = true,
                '^' => {
                    if prev[i] {
                        cur[i + 1] = true;
                        cur[i] = false;
                        cur[i - 1] = true;

                        splits += 1;
                    }
                }
                _ => (),
            }
        }

        prev = cur;
    }

    splits
}

fn part_two(input: Vec<String>) -> i64 {
    let mut prev: Vec<i64> = vec![0; input[0].len()];

    for row in input {
        let mut cur = prev.to_vec();

        for (i, c) in row.chars().enumerate() {
            match c {
                'S' => cur[i] = 1,
                '^' => {
                    cur[i + 1] += prev[i];
                    cur[i] = 0;
                    cur[i - 1] += prev[i];
                }
                _ => (),
            }
        }

        prev = cur;
    }

    prev.iter().sum()
}
