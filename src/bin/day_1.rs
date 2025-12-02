use std::fs;

fn main() {
    let file_path = "/home/golf0ned/code/projects/advent-of-code-25/src/input/day1.in";
    let input: Vec<String> = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .collect();

    println!("Part 1: {}", part_one(input.clone()));
    println!("Part 2: {}", part_two(input.clone()));
}

fn part_one(input: Vec<String>) -> i64 {
    let mut cur_rotation = 50;
    let mut num_zeros = 0;
    for line in input {
        let direction = &line[0..1];
        let amount = &line[1..].parse::<i64>().unwrap();

        if direction == "L" {
            cur_rotation -= amount;
        } else {
            cur_rotation += amount;
        }
        cur_rotation %= 100;

        if cur_rotation == 0 {
            num_zeros += 1;
        }
    }

    num_zeros
}

fn part_two(input: Vec<String>) -> i64 {
    let mut cur_rotation = 50;
    let mut num_zeros = 0;
    for line in input {
        let direction = &line[0..1];
        let amount = &line[1..].parse::<i64>().unwrap();


        let increment = if direction == "L" {
            -1
        } else {
            1
        };

        // i was lazy and i have compute
        for _ in 0..*amount {
            cur_rotation += increment;
            if cur_rotation == 0 {
                num_zeros += 1;
            } else if cur_rotation == -1 {
                cur_rotation = 99;
            } else if cur_rotation == 100 {
                num_zeros += 1;
                cur_rotation = 0;
            }
        }
    }

    num_zeros
}
