use std::collections::HashSet;

fn main() {
    let lines = io::read_lines("day11/input");
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let row = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        map.push(row);
    }
    let num_rows = map.len();
    let num_columns = map[0].len();
    let mut num_flashes = 0;
    for _ in 0..100 {
        let mut flashed = HashSet::new();
        for r in 0..num_rows {
            for c in 0..num_rows {
                map[r][c] += 1;
                if map[r][c] == 10 {
                    flashed.insert((r, c));
                }
            }
        }
        while !flashed.is_empty() {
            let mut next_flashed = HashSet::new();
            for &(r, c) in &flashed {
                let neighbors = [
                    (r - 1, c - 1),
                    (r - 1, c),
                    (r - 1, c + 1),
                    (r, c + 1),
                    (r + 1, c + 1),
                    (r + 1, c),
                    (r + 1, c - 1),
                    (r, c - 1),
                ]
                .into_iter()
                .filter(|(rr, cc)| (0..num_rows).contains(&rr) && (0..num_columns).contains(&cc));
                for (rr, cc) in neighbors {
                    if map[rr][cc] != 10 {
                        map[rr][cc] += 1;
                        if map[rr][cc] == 10 {
                            next_flashed.insert((rr, cc));
                        }
                    }
                }
            }
            flashed = next_flashed;
        }
        for r in 0..num_rows {
            for c in 0..num_rows {
                if map[r][c] == 10 {
                    map[r][c] = 0;
                    num_flashes += 1;
                }
            }
        }
    }
    println!("{}", num_flashes);
}
