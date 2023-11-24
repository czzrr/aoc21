fn main() {
    let lines = io::read_lines("day5/input");
    let vent_lines: Vec<Line> = lines.iter().map(|s| parse_line(s)).collect();

    let foo = vent_lines
        .iter()
        .map(|l| (l.start.x.max(l.end.x), l.start.y.max(l.end.y)));
    let x_dim = foo.clone().map(|(x_dim, _)| x_dim).max().unwrap() + 1;
    let y_dim = foo.map(|(_, y_dim)| y_dim).max().unwrap() + 1;

    let mut map = vec![vec![0; x_dim]; y_dim];
    for line in vent_lines {
        line.draw(&mut map);
    }
    let mut count = 0;
    for x in 0..x_dim {
        for y in 0..y_dim {
            if map[y][x] >= 2 {
                count += 1;
            }
        }
    }
    dbg!(count);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn draw(&self, map: &mut Vec<Vec<usize>>) {
        if self.start.x == self.end.x {
            let y1 = self.start.y.min(self.end.y);
            let y2 = self.start.y.max(self.end.y);
            for i in y1..y2 + 1 {
                map[i][self.start.x] += 1;
            }
        } else if self.start.y == self.end.y {
            let x1 = self.start.x.min(self.end.x);
            let x2 = self.start.x.max(self.end.x);
            for i in x1..x2 + 1 {
                map[self.start.y][i] += 1;
            }
        }
    }
}

fn parse_line(s: &str) -> Line {
    let points: Vec<_> = s.split("->").map(|s| parse_point(s)).collect();
    let start = points[0];
    let end = points[1];
    Line { start, end }
}

fn parse_point(s: &str) -> Point {
    let coords: Vec<usize> = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
    let x = coords[0];
    let y = coords[1];
    Point { x, y }
}
