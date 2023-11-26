fn main() {
    let lines = io::read_lines("day8/input");
    let four_digit_output: Vec<_> = lines
        .iter()
        .map(|s| {
            s.split('|')
                .skip(1)
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .collect::<Vec<_>>()
        })
        .collect();
    dbg!(&four_digit_output);
    let easy_digits_count: usize = four_digit_output
        .iter()
        .map(|digits| {
            digits
                .iter()
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count()
        })
        .sum();
    dbg!(easy_digits_count);
}
