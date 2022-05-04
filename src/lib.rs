use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(filename: &str) -> impl Iterator<Item=String> + '_ {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.map(|line| line.unwrap())
}