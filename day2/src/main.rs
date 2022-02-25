use aoc_io;
use std::str::FromStr;

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug)]
enum ParseCommandError {
    DirectionError,
    ValueError,
}
impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let dir = s.next().ok_or(ParseCommandError::DirectionError)?;
        let val: u32 = s
            .next()
            .ok_or(ParseCommandError::ValueError)?
            .parse()
            .map_err(|_| ParseCommandError::ValueError)?;

        match dir {
            "forward" => Ok(Command::Forward(val)),
            "down" => Ok(Command::Down(val)),
            "up" => Ok(Command::Up(val)),
            _ => Err(ParseCommandError::DirectionError),
        }
    }
}

fn main() {
    let lines: Vec<String> = aoc_io::get_input().unwrap();

    let commands: Vec<Command> = lines
        .iter()
        .map(|s| Command::from_str(s))
        .collect::<Result<Vec<Command>, ParseCommandError>>()
        .unwrap();

    // part 1
    let (pos, depth) = commands.iter().fold((0, 0), |(p, d), cmd| match cmd {
        Command::Forward(x) => (p + x, d),
        Command::Down(x) => (p, d + x),
        Command::Up(x) => (p, d - x),
    });

    println!("(pos, depth): ({}, {})", pos, depth);
    println!("pos * depth = {}", pos * depth);

    // part 2
    let (pos, depth, aim) = commands.iter().fold((0, 0, 0), |(p, d, a), cmd| match cmd {
        Command::Forward(x) => (p + x, d + a * x, a),
        Command::Down(x) => (p, d, a + x),
        Command::Up(x) => (p, d, a - x),
    });

    println!("(pos, depth, aim): ({}, {}, {})", pos, depth, aim);
    println!("pos * depth = {}", pos * depth);
}
