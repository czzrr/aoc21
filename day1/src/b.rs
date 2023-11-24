fn main() {
    let sonar_sweep: Vec<i32> = io::read_lines("day1/input")
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut num_increases = 0;
    let mut sum: i32 = sonar_sweep[0..3].iter().sum();
    for i in 3..sonar_sweep.len() {
        let new_sum = sum - sonar_sweep[i - 3] + sonar_sweep[i];
        if new_sum > sum {
            num_increases += 1;
        }
        sum = new_sum
    }
    println!("{}", num_increases);
}
