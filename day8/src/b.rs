use std::collections::{HashMap, HashSet};

fn main() {
    let lines = io::read_lines("day8/input");
    let entries: Vec<Vec<_>> = lines
        .iter()
        .map(|s| {
            s.split('|')
                .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut sum_of_values = 0;

    for entry in entries {
        //dbg!(&entry);
        let unique_signal_patterns = &entry[0];
        let (known, unknown): (Vec<&str>, _) = unique_signal_patterns
            .into_iter()
            .partition(|s| [2, 3, 4, 7].contains(&s.len()));
        //dbg!(&known);
        //dbg!(&unknown);
        let mut digit_segments: HashMap<usize, HashSet<_>> = known
            .iter()
            .map(|seg| {
                let digit = match seg.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => unreachable!(),
                };
                (digit, seg.chars().collect())
            })
            .collect();
        //dbg!(&digit_segments);

        let one_segments = digit_segments.get(&1).unwrap().clone();
        let four_segments = digit_segments.get(&4).unwrap().clone();
        for seg in unknown {
            let seg = HashSet::from_iter(seg.chars());
            if seg.len() == 5 {
                match (
                    seg.intersection(&one_segments).count(),
                    seg.intersection(&four_segments).count(),
                ) {
                    (1, 2) => digit_segments.insert(2, seg),
                    (2, 3) => digit_segments.insert(3, seg),
                    (1, 3) => digit_segments.insert(5, seg),
                    _ => unreachable!(),
                };
            } else if seg.len() == 6 {
                match (
                    seg.intersection(&one_segments).count(),
                    seg.intersection(&four_segments).count(),
                ) {
                    (2, 3) => digit_segments.insert(0, seg),
                    (1, 3) => digit_segments.insert(6, seg),
                    (2, 4) => digit_segments.insert(9, seg),
                    _ => unreachable!(),
                };
            } else {
                unreachable!()
            }
        }
        //dbg!(&digit_segments);
        let four_digit_output: Vec<usize> = entry[1]
            .iter()
            .map(|s| {
                digit_segments
                    .iter()
                    .find(|(_, seg)| **seg == s.chars().collect::<HashSet<_>>())
                    .unwrap()
                    .0
            })
            .map(|digit| *digit)
            .collect();
        dbg!(&four_digit_output);
        let value = four_digit_output
            .iter()
            .fold((1000, 0), |(f, s), d| (f / 10, s + f * d))
            .1;
        dbg!(value);
        sum_of_values += value;
    }
    dbg!(sum_of_values);
}
