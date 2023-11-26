use std::collections::{HashSet, VecDeque};

fn main() {
    let lines = io::read_lines("day9/input");
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in lines {
        let row = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        map.push(row);
    }
    for row in &map {
        println!("{:?}", row);
    }
    let m = map.len();
    let n = map[0].len();
    let mut low_points = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if neighbors(i, j, m, n)
                .into_iter()
                .all(|(ii, jj)| map[ii][jj] > map[i][j])
            {
                low_points.push((i, j));
            }
        }
    }
    let sum_of_risk_levels = low_points.iter().fold(0, |acc, h| map[h.0][h.1] + 1 + acc);
    dbg!(sum_of_risk_levels);

    let mut basins = Vec::new();

    // Perform BFS on each low point, only exploring for increasing points
    for low_point in low_points {
        let mut basin = vec![low_point];
        let mut queue = VecDeque::from([low_point]);
        let mut marked = HashSet::from([low_point]);
        while !queue.is_empty() {
            let (i, j) = queue.pop_front().unwrap();
            // If a neighbor is a higher point, add it to the basin and to the queue for further exploration.
            for (ii, jj) in neighbors(i, j, m, n) {
                if !marked.contains(&(ii, jj)) && map[ii][jj] < 9 && map[ii][jj] > map[i][j] {
                    basin.push((ii, jj));
                    queue.push_back((ii, jj));
                    marked.insert((ii, jj));
                }
            }
        }
        // dbg!(low_point);
        // dbg!(basin.len());
        basins.push(basin);
    }
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let three_largest_basins: Vec<_> = basins.iter().take(3).collect();
    let answer = three_largest_basins.iter().fold(1, |acc, b| acc * b.len());
    dbg!(answer);
}

fn neighbors(i: usize, j: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let ii = i as i32 + di;
        let jj = j as i32 + dj;
        if 0 <= ii && ii < m as i32 && 0 <= jj && jj < n as i32 {
            neighbors.push((ii as usize, jj as usize));
        }
    }
    neighbors
}
