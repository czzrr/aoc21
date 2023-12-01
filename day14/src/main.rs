use std::collections::HashMap;

fn main() {
    let lines = io::read_lines("day14/input");

    // // Part 1
    // let template = &lines[0];
    // println!("{}", template);
    // let mut rules = HashMap::new();
    // for line in &lines[2..] {
    //     let rule: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
    //     println!("{:?}", rule);
    //     rules.insert(rule[0].to_owned(), rule[1].to_owned());
    // }
    // let mut template = template.clone();
    // for _ in 0..40 {
    //     let mut next_polymer = String::new();
    //     let polymer = template.as_bytes();
    //     let mut inserted = false;
    //     for i in 1..polymer.len() {
    //         let c1 = polymer[i - 1] as char;
    //         let c2 = polymer[i] as char;
    //         let mut from = c1.to_string();
    //         from.push(c2);
    //         match rules.get(&from) {
    //             Some(to) => {
    //                 if !inserted {
    //                     next_polymer.push(c1);
    //                 }
    //                 next_polymer.push_str(to);
    //                 next_polymer.push(c2);
    //                 inserted = true;
    //             }
    //             None => inserted = false,
    //         }
    //     }
    //     template = next_polymer;
    // }
    // let mut count_map: HashMap<char, usize> = HashMap::new();
    // for c in template.chars() {
    //     match count_map.get_mut(&c) {
    //         Some(n) => *n += 1,
    //         None => {
    //             count_map.insert(c, 1);
    //         }
    //     };
    // }
    // let mut min = None;
    // let mut max = None;
    // for (k, v) in count_map {
    //     match min {
    //         None => min = Some((k, v)),
    //         Some((_, vv)) if v < vv => {
    //             min = Some((k, v));
    //         }
    //         _ => (),
    //     }
    //     match max {
    //         None => max = Some((k, v)),
    //         Some((_, vv)) if v > vv => {
    //             max = Some((k, v));
    //         }
    //         _ => (),
    //     }
    // }
    // dbg!(min);
    // dbg!(max);
    // let r = max.unwrap().1 - min.unwrap().1;
    // println!("{}", r);

    // Part 2
    let mut symbol_count_map: HashMap<String, usize> = HashMap::new();
    let mut letter_count_map = HashMap::new();
    let template = &lines[0];
    for c in template.chars() {
        match letter_count_map.get_mut(&c) {
            Some(n) => *n += 1,
            None => {
                letter_count_map.insert(c, 1);
            }
        };
    }
    let template = template.as_bytes();
    let mut rules = HashMap::new();
    for line in &lines[2..] {
        let rule: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
        rules.insert(rule[0].to_owned(), rule[1].chars().next().unwrap());
    }
    for i in 0..template.len() - 1 {
        let s = String::from_utf8(template[i..i + 2].to_vec()).unwrap();
        inc_or_init(&mut symbol_count_map, s, 1);
    }
    //dbg!(&symbol_count_map);
    for _ in 0..40 {
        //dbg!(i);
        let v: HashMap<_, usize> = symbol_count_map
            .clone()
            .into_iter()
            .filter(|(_, c)| *c != 0)
            .collect();
        //dbg!(&v);
        for (from, inc) in &v {
            if *inc == 0 {
                continue;
            }
            //dbg!(&from);
            let from_bytes = from.as_bytes();
            match rules.get(from) {
                Some(to) => {
                    *symbol_count_map.get_mut(from).unwrap() -= inc;
                    match letter_count_map.get_mut(to) {
                        Some(n) => *n += inc,
                        None => {
                            letter_count_map.insert(*to, *inc);
                        }
                    };

                    let s1 = String::from_iter([from_bytes[0] as char, *to]);
                    let s2 = String::from_iter([*to, from_bytes[1] as char]);
                    //dbg!(&s1);
                    //dbg!(&s2);
                    inc_or_init(&mut symbol_count_map, s1.clone(), *inc);
                    inc_or_init(&mut symbol_count_map, s2.clone(), *inc);

                    //dbg!(&symbol_count_map);
                    //dbg!(&letter_count_map);
                }
                _ => (),
            }
        }
    }
    // dbg!(&symbol_count_map);
    // dbg!(&letter_count_map);
    let min = *letter_count_map
        .iter()
        .min_by(|(_, c1), (_, c2)| c1.cmp(c2))
        .unwrap()
        .1;
    let max = *letter_count_map
        .iter()
        .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
        .unwrap()
        .1;
    let result = max - min;
    println!("{}", result);
}

fn inc_or_init(count_map: &mut HashMap<String, usize>, s: String, inc: usize) {
    match count_map.get_mut(&s) {
        Some(count) => *count += inc,
        None => {
            count_map.insert(s, inc);
        }
    }
}
