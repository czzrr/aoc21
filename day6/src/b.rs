fn main() {
    let lines = io::read_lines("day6/test");
    let state: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut count = 0;
    for n in state {
        let mut local_state = vec![n];
        for _ in 0..256 {
            for i in 0..local_state.len() {
                if local_state[i] == 0 {
                    local_state[i] = 6;
                    local_state.push(8);
                } else {
                    local_state[i] -= 1;
                }
            }
        }
        count += local_state.len();
    }
    println!("{}", count);
}
