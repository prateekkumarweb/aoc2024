use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let grid = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.is_alphanumeric() {
                antennas.entry(cell).or_insert(vec![]).push((i, j));
            }
        }
    }
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for (&p1, &p2) in positions.iter().tuple_combinations() {
            let (i1, j1) = p1;
            let (i2, j2) = p2;
            let i1 = i1 as isize;
            let j1 = j1 as isize;
            let i2 = i2 as isize;
            let j2 = j2 as isize;
            let a1 = (i1 - (i2 - i1), j1 - (j2 - j1));
            let a2 = (i2 + (i2 - i1), j2 + (j2 - j1));
            if a1.0 >= 0 && a1.1 >= 0 && a1.0 < grid.len() as isize && a1.1 < grid[0].len() as isize
            {
                antinodes.insert(a1);
            }
            if a2.0 >= 0 && a2.1 >= 0 && a2.0 < grid.len() as isize && a2.1 < grid[0].len() as isize
            {
                antinodes.insert(a2);
            }
        }
    }

    println!("{}", antinodes.len());
}
