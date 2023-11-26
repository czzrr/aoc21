fn main() {
    let lines = io::read_lines("day10/input");
    let mut score = 0;
    for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
            } else {
                let opening = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => unreachable!(),
                };
                if stack.pop().unwrap() != opening {
                    let points = match c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => unreachable!(),
                    };
                    score += points;
                }
            }
        }
    }
    dbg!(score);
}
