fn main() {
    let instructions = io::read_lines("day2/input");
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for instr in instructions {
        let instr: Vec<_> = instr.split_ascii_whitespace().collect();
        let direction = instr[0];
        let value: u32 = instr[1].parse().unwrap();
        match direction {
            "forward" => horizontal_pos += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => unreachable!(),
        };
    }
    println!("{}", horizontal_pos * depth);
}
