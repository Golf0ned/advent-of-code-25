use aoc25::util;
use std::cmp::{Ordering, *};
use std::collections::{BinaryHeap, HashSet};

fn main() {
    let input = util::read_comma_separated();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

struct Edge {
    size: f64,
    lower_node: usize,
    higher_node: usize,
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
            && self.lower_node == other.lower_node
            && self.higher_node == other.higher_node
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.size.total_cmp(&self.size)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn make_sorted_edges(input: &Vec<Vec<String>>) -> BinaryHeap<Edge> {
    let mut pq = BinaryHeap::new();

    for i in 0..input.len() - 1 {
        for j in (i + 1)..input.len() {
            let first = &input[i];
            let second = &input[j];

            let distance = f64::sqrt(
                first
                    .iter()
                    .zip(second.iter())
                    .map(|(a, b)| {
                        let a_num = a.parse::<i64>().unwrap();
                        let b_num = b.parse::<i64>().unwrap();

                        (a_num - b_num).pow(2)
                    })
                    .sum::<i64>() as f64,
            );

            let edge = Edge {
                size: distance,
                lower_node: i,
                higher_node: j,
            };

            pq.push(edge);
        }
    }

    pq
}

fn part_one(input: &Vec<Vec<String>>) -> i64 {
    let mut pq = make_sorted_edges(input);
    let mut circuits: Vec<HashSet<usize>> = vec![];

    for _ in 0..1000 {
        let edge = pq.pop().unwrap();
        let lower = edge.lower_node;
        let higher = edge.higher_node;

        let mut circuit_a = None;
        let mut circuit_b = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&lower) {
                circuit_a = Some(i);
            }
            if circuit.contains(&higher) {
                circuit_b = Some(i);
            }
        }

        if circuit_a.is_some() && circuit_b.is_some() {
            if circuit_a == circuit_b {
                continue;
            }
            let lower = min(circuit_a.unwrap(), circuit_b.unwrap());
            let higher = max(circuit_a.unwrap(), circuit_b.unwrap());
            let to_merge = circuits.remove(higher);
            circuits[lower].extend(to_merge);
        } else if circuit_a.is_some() {
            circuits[circuit_a.unwrap()].insert(higher);
        } else if circuit_b.is_some() {
            circuits[circuit_b.unwrap()].insert(lower);
        } else {
            circuits.push(HashSet::from([lower, higher]));
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    circuits
        .iter()
        .take(3)
        .fold(1, |acc, c| acc * c.len() as i64)
}

fn part_two(input: &Vec<Vec<String>>) -> i64 {
    let mut pq = make_sorted_edges(input);
    let mut circuits: Vec<HashSet<usize>> = vec![];

    let mut last_x = 0;

    let mut longest_connected = 1;
    while longest_connected < input.len() {
        let edge = pq.pop().unwrap();
        let lower = edge.lower_node;
        let higher = edge.higher_node;
        last_x = input[edge.lower_node][0].parse::<i64>().unwrap()
            * input[edge.higher_node][0].parse::<i64>().unwrap();

        let mut some_lower = None;
        let mut some_higher = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&lower) {
                some_lower = Some(i);
            }
            if circuit.contains(&higher) {
                some_higher = Some(i);
            }
        }

        if some_lower.is_some() && some_higher.is_some() && some_lower != some_higher {
            let first = min(some_lower.unwrap(), some_higher.unwrap());
            let second = max(some_lower.unwrap(), some_higher.unwrap());

            let to_merge = circuits.remove(second);
            circuits[first].extend(to_merge);

            longest_connected = max(longest_connected, circuits[first].len())
        } else if some_lower.is_some() {
            let lower_circuit = &mut circuits[some_lower.unwrap()];
            lower_circuit.insert(higher);

            longest_connected = max(longest_connected, lower_circuit.len())
        } else if some_higher.is_some() {
            let higher_circuit = &mut circuits[some_higher.unwrap()];
            higher_circuit.insert(lower);

            longest_connected = max(longest_connected, higher_circuit.len())
        } else {
            circuits.push(HashSet::from([lower, higher]));

            longest_connected = max(longest_connected, 2)
        }
    }

    last_x
}
