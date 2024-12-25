use itertools::Itertools;

fn main() {
    let grid = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                result += search(&grid, i, j, 0);
            }
        }
    }
    println!("{}", result);
}

fn neighbours(grid: &[Vec<u32>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i > 0 {
        result.push((i - 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if i < grid.len() - 1 {
        result.push((i + 1, j));
    }
    if j < grid[0].len() - 1 {
        result.push((i, j + 1));
    }
    result
}

fn search(grid: &[Vec<u32>], i: usize, j: usize, start: u32) -> u32 {
    if grid[i][j] == 9 {
        return 1;
    }
    let neighbours = neighbours(grid, i, j);
    neighbours
        .into_iter()
        .filter(|n| grid[n.0][n.1] == start + 1)
        .map(|(i, j)| search(grid, i, j, start + 1))
        .sum()
}
