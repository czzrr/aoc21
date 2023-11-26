fn main() {
    let lines = io::read_lines("day7/input");
    let positions: Vec<usize> = lines[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut min_fuel_cost = usize::MAX;
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();
    for n in min_pos..=max_pos {
        let fuel_cost = positions
            .iter()
            .fold(0, |fuel_cost, pos| fuel_cost + pos.abs_diff(n));
        min_fuel_cost = min_fuel_cost.min(fuel_cost);
    }
    println!("{}", min_fuel_cost);
}
