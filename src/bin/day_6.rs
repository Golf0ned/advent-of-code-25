use aoc25::util;

fn main() {
    println!("Part 1: {}", part_one(util::read_whitespace_separated()));
    println!("Part 2: {}", part_two(util::read_chars()));
}

fn part_one(input: Vec<Vec<String>>) -> i64 {
    let mut ops = vec![];
    for op in &input[input.len() - 1] {
        ops.push(op == "+");
    }

    let mut solutions: Vec<_> = input[0].iter().map(|s| s.parse::<i64>().unwrap()).collect();
    for i in 1..input.len() - 1 {
        for (j, val) in input[i].iter().enumerate() {
            if ops[j] {
                solutions[j] += val.parse::<i64>().unwrap();
            } else {
                solutions[j] *= val.parse::<i64>().unwrap();
            }
        }
    }

    solutions.iter().sum()
}

fn part_two(input: Vec<Vec<char>>) -> i64 {
    let rows = input.len();
    let cols = input[0].len();

    let mut grand_total = 0;
    let mut cur_problem = 0;

    let mut is_add = true;
    for c in 0..cols {
        let mut cur = "".to_string();
        for r in 0..rows - 1 {
            cur.push(input[r][c]);
        }
        let cur_num = cur.trim().parse::<i64>().unwrap_or(-1);

        if cur_num == -1 {
            continue;
        }

        match input[rows - 1][c] {
            '+' => {
                grand_total += cur_problem;
                cur_problem = cur_num;
                is_add = true;
            }
            '*' => {
                grand_total += cur_problem;
                cur_problem = cur_num;
                is_add = false;
            }
            _ => {
                if is_add {
                    cur_problem += cur_num;
                } else {
                    cur_problem *= cur_num;
                }
            }
        }
    }

    grand_total + cur_problem
}
