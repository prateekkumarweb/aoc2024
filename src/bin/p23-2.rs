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
    let vertices = connected.keys().cloned().collect_vec();
    let mut cliques = HashSet::from_iter(input.iter().map(|(a, b)| vec![a.clone(), b.clone()]));
    let highest_degree = connected.values().map(|v| v.len()).max().unwrap();
    for _ in 3..=highest_degree {
        let mut current_cliques = HashSet::new();
        for clique in &cliques {
            for vertex in &vertices {
                if !clique.contains(vertex) {
                    let mut is_clique = true;
                    for v in clique {
                        if !connected[v].contains(vertex) {
                            is_clique = false;
                            break;
                        }
                    }
                    if is_clique {
                        let mut new_clique = clique.clone();
                        new_clique.push(vertex.clone());
                        new_clique.sort();
                        current_cliques.insert(new_clique);
                    }
                }
            }
        }
        cliques = current_cliques;
        if cliques.is_empty() {
            break;
        }
    }
    let mut clique = cliques.into_iter().next().unwrap();
    clique.sort();
    println!("{}", clique.join(","));
}
