fn main() {
    let lines = io::read_lines("day10/input");
    let mut scores = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        let mut incomplete = true;
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
                if *stack.last().unwrap() == opening {
                    stack.pop();
                } else {
                    incomplete = false;
                    break;
                }
            }
        }
        let mut score: usize = 0;
        if incomplete {
            stack.reverse();
            for c in stack {
                let points = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
                score *= 5;
                score += points;
            }
            scores.push(score);
        }
    }
    scores.sort();
    println!("{:?}", scores);
    println!("{}", scores[scores.len() / 2]);
}
