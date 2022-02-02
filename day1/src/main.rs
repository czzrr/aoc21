use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let v = get_input();
    // part 1
    let n = v
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(x, y)| x < y)
        .count();
    // part 2
    let m = v
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows::<(_, _)>()
        .filter(|(x, y)| x < y)
        .count();
    println!("{:?}", n);
    println!("{:?}", m);
}

fn get_input() -> Vec<i32> {
    let file = File::open("input.txt").unwrap();
    let v = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();
    v
}
