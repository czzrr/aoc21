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
            let mut low_point = true;
            for (di, dj) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                let ii = i as i32 + di;
                let jj = j as i32 + dj;
                if 0 <= ii
                    && ii < m as i32
                    && 0 <= jj
                    && jj < n as i32
                    && map[ii as usize][jj as usize] <= map[i][j]
                {
                    low_point = false;
                }
            }
            if low_point {
                low_points.push((i, j));
            }
        }
    }
    let sum_of_risk_levels = low_points.iter().fold(0, |acc, h| map[h.0][h.1] + 1 + acc);
    dbg!(sum_of_risk_levels);
}
