use std::fs;

fn main() {
    let file_path = "/home/golf0ned/code/projects/advent-of-code-25/src/input/day4.in";
    let input: Vec<Vec<char>> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .map(|s| s.chars().collect())
        .collect();

    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(&mut input.clone()));
}

fn part_one(input: Vec<Vec<char>>) -> i64 {
    let mut total = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '.' {
                continue;
            }

            let mut cur_count = 0;

            for ii in -1..=1 {
                for jj in -1isize..=1isize {
                    if ii == 0 && jj == 0 {
                        continue;
                    }

                    let r = i as isize + ii;
                    let c = j as isize + jj;

                    if r < 0 || r >= input.len() as isize || c < 0 || c >= row.len() as isize {
                        continue;
                    }

                    if input[r as usize][c as usize] == '@' {
                        cur_count += 1;
                    }
                }
            }

            if cur_count < 4 {
                total += 1;
            }
        }
    }

    total
}

fn part_two(input: &mut Vec<Vec<char>>) -> i64 {
    let mut total = 0;
    let mut cur_total = -1;
    while cur_total != 0 {
        cur_total = 0;

        for (i, row) in input.clone().iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == '.' {
                    continue;
                }

                let mut cur_count = 0;

                for ii in -1..=1 {
                    for jj in -1isize..=1isize {
                        if ii == 0 && jj == 0 {
                            continue;
                        }

                        let r = i as isize + ii;
                        let c = j as isize + jj;

                        if r < 0 || r >= input.len() as isize || c < 0 || c >= row.len() as isize {
                            continue;
                        }

                        if input[r as usize][c as usize] == '@' {
                            cur_count += 1;
                        }
                    }
                }

                if cur_count < 4 {
                    cur_total += 1;
                    input[i][j] = '.';
                }
            }
        }

        total += cur_total;
    }

    total
}
