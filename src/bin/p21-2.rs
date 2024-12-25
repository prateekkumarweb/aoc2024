use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::io::stdin().lines().map(|l| l.unwrap()).collect_vec();
    let mut result = 0;
    let find_all_moves = find_all_moves();
    let all_num_moves = find_all_moves
        .values()
        .flatten()
        .map(|x| x.iter().collect::<String>());
    let all_arr_moves = ("^<>vA".chars())
        .cartesian_product("^<>vA".chars())
        .flat_map(|(a, b)| move_char(a, b))
        .map(|s| s.to_string());
    let all_moves = all_num_moves.chain(all_arr_moves);

    let mut memo = HashMap::new();

    let n_steps = 24;

    for code in all_moves {
        for i in 0..n_steps {
            find_min_len_code(&code, i, &mut memo);
        }
    }

    for code in input {
        let steps = short_len_num_code(&code, &find_all_moves);
        let len = steps
            .into_iter()
            .map(|code| {
                let mut len = 0;
                let mut code2 = code.chars().rev().collect_vec();
                let mut start = 'A';
                while let Some(end) = code2.pop() {
                    let possible_moves = move_char(start, end);
                    len += possible_moves
                        .iter()
                        .map(|s| find_min_len_code(s, n_steps, &mut memo))
                        .min()
                        .unwrap();
                    start = end;
                }
                len
            })
            .min()
            .unwrap();

        result += len
            * code
                .strip_suffix("A")
                .unwrap()
                .trim_start_matches("0")
                .parse::<usize>()
                .unwrap();
    }
    println!("{}", result);
}

fn short_len_num_code(code: &str, moves: &HashMap<(char, char), Vec<Vec<char>>>) -> Vec<String> {
    let mut code = code.chars().rev().collect_vec();
    let mut start = 'A';
    let mut possible = vec!["".to_string()];
    while let Some(end) = code.pop() {
        let possible_moves = moves.get(&(start, end)).unwrap();
        possible = possible
            .into_iter()
            .cartesian_product(possible_moves.iter())
            .map(|(mut a, b)| {
                a.extend(b);
                a
            })
            .collect_vec();
        start = end;
    }
    possible
}

fn find_min_len_code(
    code: &str,
    steps: usize,
    memo: &mut HashMap<(String, usize), usize>,
) -> usize {
    if steps == 0 {
        return code.len();
    }
    if let Some(c) = memo.get(&(code.to_string(), steps)) {
        return *c;
    }
    let next_steps = short_len_code(code);
    let min_len = next_steps
        .into_iter()
        .map(|s| {
            s.into_iter()
                .map(|s| find_min_len_code(s, steps - 1, memo))
                .min()
                .unwrap()
        })
        .sum();
    memo.insert((code.to_string(), steps), min_len);
    min_len
}

fn short_len_code(code: &str) -> Vec<Vec<&str>> {
    let mut start = 'A';
    let mut possible = vec![];
    let mut code = code.chars().rev().collect_vec();
    while let Some(end) = code.pop() {
        let possible_moves = move_char(start, end);
        possible.push(possible_moves);
        start = end;
    }
    possible
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
//
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

fn move_char(start: char, end: char) -> Vec<&'static str> {
    match (start, end) {
        ('^', '^') => vec!["A"],
        ('^', 'v') => vec!["vA"],
        ('^', '<') => vec!["v<A"],
        ('^', '>') => vec!["v>A"],
        ('^', 'A') => vec![">A"],

        ('v', '^') => vec!["^A"],
        ('v', 'v') => vec!["A"],
        ('v', '<') => vec!["<A"],
        ('v', '>') => vec![">A"],
        ('v', 'A') => vec!["^>A", ">^A"],

        ('<', '^') => vec![">^A"],
        ('<', 'v') => vec![">A"],
        ('<', '<') => vec!["A"],
        ('<', '>') => vec![">>A"],
        ('<', 'A') => vec![">^>A", ">>^A"],

        ('>', '^') => vec!["<^A", "^<A"],
        ('>', 'v') => vec!["<A"],
        ('>', '<') => vec!["<<A"],
        ('>', '>') => vec!["A"],
        ('>', 'A') => vec!["^A"],

        ('A', '^') => vec!["<A"],
        ('A', 'v') => vec!["<vA", "v<A"],
        ('A', '<') => vec!["<v<A", "v<<A"],
        ('A', '>') => vec!["vA"],
        ('A', 'A') => vec!["A"],

        _ => panic!("Invalid move"),
    }
}

fn find_all_moves() -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut moves = HashMap::new();
    let grid = [vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['x', '0', 'A']];

    for p1 in 0..12 {
        for p2 in 0..12 {
            let start = (p1 / 3, p1 % 3);
            let end = (p2 / 3, p2 % 3);
            let start_p = grid[start.0][start.1];
            let end_p = grid[end.0][end.1];
            if start_p == 'x' || end_p == 'x' {
                continue;
            }
            if start_p == end_p {
                moves.insert((start_p, end_p), vec![vec!['A']]);
                continue;
            }
            let possible = if start.0 > end.0 {
                vec!['^'; start.0 - end.0]
            } else {
                vec!['v'; end.0 - start.0]
            };
            let possible = if start.1 > end.1 {
                possible
                    .into_iter()
                    .chain(vec!['<'; start.1 - end.1])
                    .collect_vec()
            } else {
                possible
                    .into_iter()
                    .chain(vec!['>'; end.1 - start.1])
                    .collect_vec()
            };
            let len = possible.len();
            let combs = possible
                .into_iter()
                .permutations(len)
                .unique()
                .filter(|c| {
                    let mut current = start;
                    for &d in c {
                        match d {
                            '^' => current.0 -= 1,
                            'v' => current.0 += 1,
                            '<' => current.1 -= 1,
                            '>' => current.1 += 1,
                            _ => panic!("Invalid direction"),
                        }
                        if grid[current.0][current.1] == 'x' {
                            return false;
                        }
                    }
                    true
                })
                .map(|mut v| {
                    v.push('A');
                    v
                })
                .collect_vec();
            moves.insert((start_p, end_p), combs);
        }
    }

    moves
}
