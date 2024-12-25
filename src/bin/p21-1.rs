use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::io::stdin().lines().map(|l| l.unwrap()).collect_vec();
    let mut result = 0;
    for code in input {
        let step1 = short_len_num_code(&code);
        let mut step2 = step1
            .into_iter()
            .flat_map(|code| short_len_code(&code))
            .collect_vec();
        step2.sort_by_key(|p| p.len());
        let least_len = step2.first().unwrap().len();
        step2 = step2
            .into_iter()
            .filter(|p| p.len() == least_len)
            .collect_vec();
        let mut step3 = step2
            .into_iter()
            .flat_map(|code| short_len_code(&code))
            .collect_vec();
        step3.sort_by_key(|p| p.len());
        let len = step3.first().unwrap().len();

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

fn short_len_num_code(code: &str) -> Vec<String> {
    let moves = find_all_moves();
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
    possible.sort_by_key(|p| p.len());
    possible
}

fn short_len_code(code: &str) -> Vec<String> {
    let mut start = 'A';
    let mut possible = vec!["".to_string()];
    let mut code = code.chars().rev().collect_vec();
    while let Some(end) = code.pop() {
        let possible_moves = move_char(start, end);
        possible = possible
            .into_iter()
            .cartesian_product(possible_moves.into_iter())
            .map(|(mut a, b)| {
                a.push_str(b);
                a
            })
            .collect_vec();
        start = end;
    }
    possible.sort_by_key(|p| p.len());
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
