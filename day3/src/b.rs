fn main() {
    let lines: Vec<String> = io::read_lines("day3/input");
    let numbers: Vec<Vec<u8>> = lines.clone().into_iter().map(|s| s.into_bytes()).collect();

    let mut oxygen_nums = numbers.clone();
    let num_columns = lines[0].len();
    for bit_idx in 0..num_columns {
        let oxygen_chosen_bit = most_common(oxygen_nums.iter().map(|num| num[bit_idx]));
        oxygen_nums.retain(|num| num[bit_idx] == oxygen_chosen_bit);

        if oxygen_nums.len() == 1 {
            break;
        }
    }
    let oxygen_gen_rating =
        usize::from_str_radix(&String::from_utf8(oxygen_nums[0].clone()).unwrap(), 2).unwrap();
    println!("Oxygen generator rating: {}", oxygen_gen_rating);

    let mut co2_nums = numbers.clone();
    for bit_idx in 0..lines[0].len() {
        let co2_chosen_bit = least_common(co2_nums.iter().map(|num| num[bit_idx]));
        co2_nums.retain(|num| num[bit_idx] == co2_chosen_bit);

        if co2_nums.len() == 1 {
            break;
        }
    }
    let co2_scrubber_rating =
        usize::from_str_radix(&String::from_utf8(co2_nums[0].clone()).unwrap(), 2).unwrap();
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);

    println!("{}", oxygen_gen_rating * co2_scrubber_rating);
}

fn least_common(it: impl IntoIterator<Item = u8>) -> u8 {
    let (zeros, ones) = count_bits(it);
    if zeros <= ones {
        b'0'
    } else {
        b'1'
    }
}

fn most_common(it: impl IntoIterator<Item = u8>) -> u8 {
    let (zeros, ones) = count_bits(it);
    if ones >= zeros {
        b'1'
    } else {
        b'0'
    }
}

fn count_bits(bits_iter: impl IntoIterator<Item = u8>) -> (u32, u32) {
    bits_iter.into_iter().fold((0, 0), |(zeros, ones), bit| {
        if bit == b'0' {
            (zeros + 1, ones)
        } else {
            (zeros, ones + 1)
        }
    })
}
