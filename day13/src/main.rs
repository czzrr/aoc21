fn main() {
    let lines = io::read_lines("day13/input");
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut lines = lines.iter();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        //dbg!(line);
        let mut point = line.split(',').map(|s| s.parse().unwrap());
        let x = point.next().unwrap();
        let y = point.next().unwrap();
        points.push((x, y));
    }
    let x_dim = 1 + points.iter().max_by(|p1, p2| p1.0.cmp(&p2.0)).unwrap().0;
    let y_dim = 1 + points.iter().max_by(|p1, p2| p1.1.cmp(&p2.1)).unwrap().1;
    dbg!(x_dim);
    dbg!(y_dim);
    let mut paper = vec![vec!['.'; x_dim]; y_dim];
    for (x, y) in points {
        paper[y][x] = '#';
    }
    //print_paper(&paper, x_dim, y_dim);

    let mut x_end = x_dim;
    let mut y_end = y_dim;

    for line in lines {
        let split: Vec<_> = line.split('=').collect();
        let coord: usize = split[1].parse().unwrap();
        let axis = *split[0].as_bytes().last().unwrap() as char;
        //println!("Fold {}={}", axis, coord);
        match axis {
            'x' => {
                for x in coord + 1..x_end {
                    for y in 0..y_end {
                        if paper[y][x] == '#' {
                            paper[y][coord - (x - coord)] = '#';
                        }
                    }
                }
                x_end = coord;
            }
            'y' => {
                for x in 0..x_end {
                    for y in coord + 1..y_end {
                        if paper[y][x] == '#' {
                            paper[coord - (y - coord)][x] = '#';
                        }
                    }
                }
                y_end = coord;
            }
            _ => unreachable!(),
        }
    }

    // Part 1
    let mut count = 0;
    for x in 0..x_end {
        for y in 0..y_end {
            if paper[y][x] == '#' {
                count += 1;
            }
        }
    }
    println!("{}", count);

    // Part 2
    print_paper(&paper, x_end, y_end);
}

fn print_paper(paper: &Vec<Vec<char>>, x_end: usize, y_end: usize) {
    println!("Paper:");
    for y in 0..y_end {
        for x in 0..x_end {
            print!("{}", paper[y][x]);
        }
        println!();
    }
}
