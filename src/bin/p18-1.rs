use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut splits = l.split(',').map(|s| s.parse::<usize>().unwrap());
            (splits.next().unwrap(), splits.next().unwrap())
        })
        .collect_vec();

    let size = if input.len() == 25 { 7 } else { 71 };
    let bytes = if input.len() == 25 { 12 } else { 1024 };
    let mut grid = vec![vec![0; size]; size];
    for &(i, j) in &input[..bytes] {
        grid[i][j] = -1;
    }
    let start = (0, 0);
    let end = (size - 1, size - 1);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    while let Some((i, j)) = queue.pop_front() {
        if (i, j) == end {
            break;
        }
        for (ni, nj) in neighbors(i, j, size) {
            if grid[ni][nj] == 0 {
                grid[ni][nj] = grid[i][j] + 1;
                queue.push_back((ni, nj));
            }
        }
    }
    println!("{}", grid[end.0][end.1]);
}

fn neighbors(i: usize, j: usize, size: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if i > 0 {
        result.push((i - 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if i + 1 < size {
        result.push((i + 1, j));
    }
    if j + 1 < size {
        result.push((i, j + 1));
    }
    result
}
