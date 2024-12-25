use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut parts = l.split("-");
            (
                parts.next().unwrap().trim().to_string(),
                parts.next().unwrap().trim().to_string(),
            )
        })
        .collect_vec();
    let mut connected = HashMap::new();
    for (a, b) in &input {
        let a_val = connected.entry(a.clone()).or_insert_with(HashSet::new);
        a_val.insert(b.clone());
        let b_val = connected.entry(b.clone()).or_insert_with(HashSet::new);
        b_val.insert(a.clone());
    }
    let mut visited = HashSet::new();
    for (a, b) in &input {
        let a_map = connected.get(a).unwrap();
        let b_map = connected.get(b).unwrap();
        let common = a_map.intersection(b_map).collect_vec();
        for c in common {
            let mut triad = vec![a, b, c];
            triad.sort();
            if triad.iter().any(|x| x.starts_with('t')) {
                visited.insert(triad);
            }
        }
    }
    println!("{}", visited.len());
}
