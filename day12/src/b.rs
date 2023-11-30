use std::collections::{HashMap, HashSet};

fn main() {
    let lines = io::read_lines("day12/input");
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in &lines {
        let edge: Vec<&str> = line.split('-').collect();
        dbg!(&edge);
        let a = edge[0];
        let b = edge[1];
        for (v, w) in [(a, b), (b, a)] {
            match edges.get_mut(v) {
                Some(neighbors) => neighbors.push(w),
                None => {
                    edges.insert(v, vec![w]);
                }
            };
        }
    }
    println!("{:?}", edges);
    let r = dfs("start", &edges, HashSet::new(), None);
    println!("{}", r);
}

fn dfs<'a>(
    v: &'a str,
    edges: &HashMap<&str, Vec<&str>>,
    mut visited: HashSet<&'a str>,
    double: Option<&str>,
) -> usize {
    if v == "start" && visited.contains(v) {
        return 0;
    }
    if v == "end" {
        return 1;
    }
    if v.chars().all(|c| c.is_lowercase()) {
        if visited.contains(v) && double != Some(v) {
            return 0;
        }
        visited.insert(v);
    }
    let mut num_paths = 0;
    let (visited_neighbors, unvisited_neighbors) = edges
        .get(v)
        .unwrap()
        .iter()
        .partition::<Vec<_>, _>(|w| visited.contains(**w));
    if double.is_none() {
        for w in visited_neighbors {
            num_paths += dfs(w, edges, visited.clone(), Some(w));
        }
    }
    for w in unvisited_neighbors {
        num_paths += dfs(w, &edges, visited.clone(), double);
    }
    return num_paths;
}
