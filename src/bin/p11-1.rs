use itertools::Itertools;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut stones = buf
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|s| {
                if s == 0 {
                    vec![1]
                } else if (s.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
                    let n_digits = s.checked_ilog10().unwrap_or(0) + 1;
                    vec![s / 10usize.pow(n_digits / 2), s % 10usize.pow(n_digits / 2)]
                } else {
                    vec![s * 2024]
                }
                .into_iter()
            })
            .collect_vec();
    }
    println!("{}", stones.len());
}
