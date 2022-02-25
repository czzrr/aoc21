use aoc_io;

fn main() {
    let lines: Vec<String> = aoc_io::get_input().unwrap();
    let numbers: Vec<Vec<u8>> = lines
        .iter()
        .map(|s| s.as_bytes().iter().map(|x| x - '0' as u8).collect())
        .collect();
    let bit_columns = (0..lines[0].len()).map(|i| numbers.iter().clone().map(move |num| num[i]));

    // part 1

    let counts = bit_columns.clone().map(|it| count_bits(it));

    let msbs = counts
        .clone()
        .map(|(zeros, ones)| if zeros > ones { 0 } else { 1 });

    let lsbs = counts.map(|(zeros, ones)| if zeros < ones { 0 } else { 1 });

    print!("Most significant bits: ");
    for x in msbs {
        print!("{:?}", x);
    }
    println!();

    print!("Least significant bits: ");
    for x in lsbs {
        print!("{:?}", x);
    }
    println!();

    // part 2

    let mut oxygen_nums = numbers.clone();

    for bit_idx in 0..lines[0].len() {
        let oxygen_chosen_bit = most_common_1(oxygen_nums.iter().map(|num| num[bit_idx]));
        oxygen_nums.retain(|num| num[bit_idx] == oxygen_chosen_bit);

        if oxygen_nums.len() == 1 {
            break;
        }
    }

    println!(
        "oxygen generator rating: {:x?}",
        oxygen_nums[0]
    );

    let mut co2_nums = numbers.clone();

    for bit_idx in 0..lines[0].len() {
        let co2_chosen_bit = least_common_0(co2_nums.iter().map(|num| num[bit_idx]));
        co2_nums.retain(|num| num[bit_idx] == co2_chosen_bit);

        if co2_nums.len() == 1 {
            break;
        }
    }

    println!(
        "CO2 scrubber rating: {:x?}",
        co2_nums[0]
    );
}

fn least_common_0(it: impl IntoIterator<Item = u8>) -> u8 {
    let (zeros, ones) = count_bits(it);
    if zeros <= ones {
        0
    } else {
        1
    }
}

fn most_common_1(it: impl IntoIterator<Item = u8>) -> u8 {
    let (zeros, ones) = count_bits(it);
    if ones >= zeros {
        1
    } else {
        0
    }
}

fn count_bits(bits_iter: impl IntoIterator<Item = u8>) -> (u32, u32) {
    bits_iter.into_iter().fold((0, 0), |(zeros, ones), bit| {
        if bit == 0 {
            (zeros + 1, ones)
        } else {
            (zeros, ones + 1)
        }
    })
}
