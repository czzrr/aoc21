fn main() {
    let sonar_sweep: Vec<u32> = io::read_lines("day1/input")
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut num_increases = 0;
    for i in 1..sonar_sweep.len() {
        if sonar_sweep[i] > sonar_sweep[i - 1] {
            num_increases += 1;
        }
    }
    println!("{}", num_increases);
}
