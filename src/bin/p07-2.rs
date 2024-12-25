use std::iter::repeat_n;

use itertools::Itertools;

fn main() {
    let lines = std::io::stdin().lines().map(Result::unwrap);
    let mut result = 0;
    for line in lines {
        let mut splits = line.split(':').map(|p| p.trim());
        let amount = splits.next().unwrap().parse().unwrap();
        let rest = splits.next().unwrap();
        let vals = rest
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        let ops = ['+', '*', '|'];
        for p in repeat_n(ops.iter(), vals.len() - 1).multi_cartesian_product() {
            let mut total = vals[0];
            for i in 0..p.len() {
                match p[i] {
                    '+' => total += vals[i + 1],
                    '*' => total *= vals[i + 1],
                    '|' => {
                        total = (total
                            * 10usize.pow(vals[i + 1].checked_ilog10().unwrap_or(0) + 1))
                            + vals[i + 1]
                    }
                    _ => unreachable!(),
                }
            }
            if total == amount {
                result += total;
                break;
            }
        }
    }
    println!("{}", result);
}
