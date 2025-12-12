use std::env;
use std::fs;

pub fn read_lines() -> Vec<String> {
    let file_path = env::args().nth(1).expect("File not found");
    fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_chars() -> Vec<Vec<char>> {
    let file_path = env::args().nth(1).expect("File not found");
    fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .map(|l| l.chars().collect())
        .collect()
}

pub fn read_comma_separated() -> Vec<Vec<String>> {
    let file_path = env::args().nth(1).expect("File not found");
    fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
        .map(|l| l.split(',').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

pub fn read_whitespace_separated() -> Vec<Vec<String>> {
    let file_path = env::args().nth(1).expect("File not found");
    fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}
