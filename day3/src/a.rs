fn main() {
    let lines = io::read_lines("day3/input");
    let mut gamma_rate = 0;
    for i in 0..lines[0].len() {
        let (zeros, ones) = lines.iter().fold((0, 0), |(zeros, ones), bit_string| {
            if bit_string.as_bytes()[i] == b'0' {
                (zeros + 1, ones)
            } else {
                (zeros, ones + 1)
            }
        });
        gamma_rate = gamma_rate << 1;
        if zeros < ones {
            gamma_rate |= 1;
        } else {
        }
    }
    let epsilon_rate = !gamma_rate & !(-1 << lines[0].len());
    println!("{:b}", gamma_rate);
    println!("{:b}", epsilon_rate);
    println!("{}", gamma_rate * epsilon_rate);
}
