fn main() {
    let lines = io::read_lines("day6/input");
    let mut state: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    for _ in 0..80 {
        for i in 0..state.len() {
            if state[i] == 0 {
                state[i] = 6;
                state.push(8);
            } else {
                state[i] -= 1;
            }
        }
    }
    println!("{}", state.len());
}
