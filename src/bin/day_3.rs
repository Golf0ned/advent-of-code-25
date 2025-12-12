use aoc25::util;

fn main() {
    let input = util::read_lines();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Vec<String>) -> i64 {
    let mut total = 0;

    for line in input {
        let mut first_pass = line.clone();
        first_pass.pop();

        let mut largest = 0;
        let mut largest_ind = 0;
        for (i, c) in first_pass.chars().enumerate() {
            if c.to_digit(10).unwrap() > largest {
                largest = c.to_digit(10).unwrap();
                largest_ind = i;
            }
        }

        total += largest as i64 * 10;

        let mut next_largest = 0;
        for c in line[largest_ind + 1..].chars() {
            if c.to_digit(10).unwrap() > next_largest {
                next_largest = c.to_digit(10).unwrap();
            }
        }
        total += next_largest as i64;
    }

    total
}

fn part_two(input: &Vec<String>) -> i64 {
    let mut total = 0;

    for line in input {
        let mut num_str = line.clone();
        while num_str.len() > 12 {
            let mut index = 0;
            let mut prev = 10;
            for (i, c) in num_str.chars().enumerate() {
                let cur = c.to_digit(10).unwrap();
                if cur > prev {
                    break;
                } else {
                    prev = cur;
                }
                index = i;
            }
            num_str.remove(index);
        }
        total += num_str.parse::<i64>().unwrap();
    }

    total
}
