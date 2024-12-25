use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let stones = buf
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    let mut memo = HashMap::new();
    let blinks = 75;
    let result: usize = stones
        .into_iter()
        .map(|s| count(s, blinks, &mut memo))
        .sum();
    println!("{}", result);
}

fn count(stone: usize, blinks: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let result = memo.get(&(stone, blinks));
    if let Some(result) = result {
        return *result;
    }
    let result = if blinks == 0 {
        1
    } else if stone == 0 {
        count(1, blinks - 1, memo)
    } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
        let n_digits = stone.checked_ilog10().unwrap_or(0) + 1;
        count(stone / 10usize.pow(n_digits / 2), blinks - 1, memo)
            + count(stone % 10usize.pow(n_digits / 2), blinks - 1, memo)
    } else {
        count(stone * 2024, blinks - 1, memo)
    };
    memo.insert((stone, blinks), result);
    result
}
