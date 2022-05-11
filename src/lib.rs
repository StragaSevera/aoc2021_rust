use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
}

pub fn read_string(input: &str) -> impl Iterator<Item=String> + '_ {
    input.lines().map(|e| e.to_string())
}