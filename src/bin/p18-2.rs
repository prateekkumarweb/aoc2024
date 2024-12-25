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
    let mut grid = vec![vec![0; size]; size];
    let mut found = (0, 0);
    for &(i, j) in &input {
        grid[i][j] = -1;
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
        if grid[end.0][end.1] == 0 {
            found = (i, j);
            break;
        }
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                if *cell > 0 {
                    *cell = 0;
                }
            }
        }
    }

    println!("{},{}", found.0, found.1);
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
