use std::process::exit;

fn main() {
    let lines = io::read_lines("day4/input");
    let numbers_drawn: Vec<usize> = lines[0].split(',').map(|n| n.parse().unwrap()).collect();
    let mut boards = parse_boards(&lines[1..]);
    for num in numbers_drawn {
        for board in &mut boards {
            board.mark(num);
            if board.wins() {
                let sum = board.sum();
                println!("{} * {} = {}", sum, num, sum * num);
                exit(0);
            }
        }
    }
}

#[derive(Debug)]
struct Entry {
    pub number: usize,
    pub marked: bool,
}

impl Entry {
    pub fn new(number: usize) -> Entry {
        Entry {
            number,
            marked: false,
        }
    }
}

#[derive(Debug)]
struct Board {
    entries: Vec<Vec<Entry>>,
}

impl Board {
    pub fn mark(&mut self, n: usize) {
        for row in &mut self.entries {
            for entry in row {
                if entry.number == n {
                    entry.marked = true;
                }
            }
        }
    }
    pub fn sum(&self) -> usize {
        self.entries.iter().fold(0, |sum, row| {
            row.iter()
                .map(|e| if e.marked { 0 } else { e.number })
                .sum::<usize>()
                + sum
        })
    }

    pub fn wins(&self) -> bool {
        let row_wise = self.entries.iter().any(|row| row.iter().all(|e| e.marked));
        let mut column_wise = false;
        for i in 0..5 {
            column_wise = column_wise || self.entries.iter().map(|row| row[i].marked).all(|b| b);
        }
        row_wise || column_wise
    }
}

fn parse_boards<'a>(mut lines: &[String]) -> Vec<Board> {
    let mut boards = Vec::new();
    loop {
        if lines.len() < 6 {
            break;
        }
        lines = &lines[1..];

        let board = parse_board(&lines[..5]);
        boards.push(board);
        lines = &lines[5..];
    }
    boards
}

fn parse_board(lines: &[String]) -> Board {
    let mut rows = Vec::with_capacity(5);
    for i in 0..5 {
        let row = parse_row(&lines[i]);
        rows.push(row);
    }

    Board { entries: rows }
}

fn parse_row(row: &str) -> Vec<Entry> {
    row.split_ascii_whitespace()
        .map(|s| Entry::new(s.parse().unwrap()))
        .collect()
}
