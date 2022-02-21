use std::fs::File;
use std::io::{self, BufRead};


pub fn get_input() -> Result<Vec<String>, io::Error> {
    let file = File::open("input.txt").unwrap();
    let v = io::BufReader::new(file)
        .lines()
        .collect();
    v
}