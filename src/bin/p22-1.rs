use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let result = input
        .into_iter()
        .map(gen_secret_number)
        .sum::<usize>();

    println!("{}", result);
}

fn gen_secret_number(initial: usize) -> usize {
    let mut secret = initial;
    for _ in 0..2000 {
        secret = ((secret << 6) ^ secret) % 16777216;
        secret = ((secret >> 5) ^ secret) % 16777216;
        secret = ((secret << 11) ^ secret) % 16777216;
    }
    secret
}
