use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const NSTEPS: usize = 2000;

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let result = input
        .into_iter()
        .map(gen_secret_number)
        .collect_vec();

    let mut sum_prices = HashMap::new();

    for (prices, changes) in result.iter() {
        let mut seen_changes = HashSet::new();
        for i in 3..NSTEPS {
            let change = (changes[i - 3], changes[i - 2], changes[i - 1], changes[i]);
            if !seen_changes.contains(&change) {
                seen_changes.insert(change);
                *sum_prices.entry(change).or_insert(0) += prices[i];
            }
        }
    }

    println!("{}", sum_prices.values().max().unwrap());
}

fn gen_secret_number(initial: usize) -> (Vec<i64>, Vec<i64>) {
    let mut secret = initial;
    let mut prices = vec![];
    let mut changes = vec![];
    for _ in 0..NSTEPS {
        let prev = secret;
        secret = ((secret << 6) ^ secret) % 16777216;
        secret = ((secret >> 5) ^ secret) % 16777216;
        secret = ((secret << 11) ^ secret) % 16777216;
        prices.push((secret % 10) as i64);
        changes.push((secret % 10) as i64 - (prev % 10) as i64);
    }
    (prices, changes)
}
