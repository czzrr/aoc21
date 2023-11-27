use std::collections::HashMap;

fn main() {
    let lines = io::read_lines("day6/input");
    let initial_state: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut count_map: HashMap<usize, usize> = HashMap::from_iter((0..=8).map(|n| (n, 0)));
    for n in initial_state {
        *count_map.get_mut(&n).unwrap() += 1;
    }
    for _ in 0..256 {
        let mut next_count_map = count_map.clone();
        for (n, c) in count_map.iter() {
            *next_count_map.get_mut(n).unwrap() -= c;
            if *n == 0 {
                *next_count_map.get_mut(&6).unwrap() += c;
                *next_count_map.get_mut(&8).unwrap() += c;
            } else {
                *next_count_map.get_mut(&(n - 1)).unwrap() += c;
            }
        }
        count_map = next_count_map;
    }
    let count: usize = count_map.values().sum();
    println!("{}", count);
}
