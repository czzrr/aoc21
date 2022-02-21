use itertools::Itertools;
use aoc_io;

fn main() {
    let v: Vec<i32> = aoc_io::get_input().unwrap().iter().map(|s| s.parse::<i32>().unwrap()).collect();
    
    // part 1
    let n = v
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(x, y)| x < y)
        .count();
    println!("{:?}", n);

    // part 2
    let m = v
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(x, y, z)| x + y + z)
        .tuple_windows::<(_, _)>()
        .filter(|(x, y)| x < y)
        .count();
    println!("{:?}", m);
}