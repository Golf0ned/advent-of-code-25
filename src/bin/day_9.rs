use aoc25::util;
use std::cmp::*;

fn main() {
    let input = util::read_comma_separated();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Vec<Vec<String>>) -> i64 {
    let mut largest_area = 0;

    for i in 0..input.len() - 1 {
        for j in (i + 1)..input.len() {
            let first = &input[i];
            let first_x = first[0].parse::<i64>().unwrap();
            let first_y = first[1].parse::<i64>().unwrap();

            let second = &input[j];
            let second_x = second[0].parse::<i64>().unwrap();
            let second_y = second[1].parse::<i64>().unwrap();

            largest_area = max(
                largest_area,
                ((first_x - second_x + 1) * (first_y - second_y + 1)).abs(),
            )
        }
    }

    largest_area
}

fn part_two(input: &Vec<Vec<String>>) -> i64 {
    0
}
