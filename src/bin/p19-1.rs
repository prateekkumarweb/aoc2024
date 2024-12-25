use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap());
    let line = lines.next().unwrap();
    let mut patterns = line.trim().split(", ").collect_vec();
    patterns.sort();
    lines.next().unwrap();
    let mut count = 0;
    let mut cache = HashMap::new();
    let lines = lines.collect_vec();
    for design in &lines {
        if is_possible_design(design, &patterns, &mut cache) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn is_possible_design<'a, 'p>(
    design: &'a str,
    patterns: &'p [&'p str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&found) = cache.get(design) {
        return found;
    }
    for pat in patterns {
        if let Some(rest) = design.strip_prefix(pat) {
            let found = is_possible_design(rest, patterns, cache);
            if found {
                cache.insert(design, true);
                return true;
            }
        }
    }
    cache.insert(design, false);
    false
}
