use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Item>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

fn main() {
    let grid = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect_vec())
        .collect_vec();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let grid = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &c)| match c {
                    '.' => Item::Empty,
                    '#' => Item::Wall,
                    'S' => {
                        start = (i, j);
                        Item::Empty
                    }
                    'E' => {
                        end = (i, j);
                        Item::Empty
                    }
                    _ => panic!("Invalid character at ({}, {}): {}", i, j, c),
                })
                .collect_vec()
        })
        .collect_vec();
    let grid = Grid { grid };
    let distances = find_distances(&grid, start, end);
    let min_distance = distances[end.0][end.1].0;
    let mut savings = HashMap::new();
    for (i, row) in distances.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid.grid[i][j] == Item::Wall {
                if i > 0 && i + 1 < grid.height() {
                    let (up, down) = (distances[i - 1][j].0, distances[i + 1][j].1);
                    if up != usize::MAX && down != usize::MAX && up + down + 2 < min_distance {
                        let saved = min_distance - (up + down + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                    let (down, up) = (distances[i + 1][j].0, distances[i - 1][j].1);
                    if up != usize::MAX && down != usize::MAX && up + down + 2 < min_distance {
                        let saved = min_distance - (up + down + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                }
                if j > 0 && j + 1 < grid.width() {
                    let (left, right) = (distances[i][j - 1].0, distances[i][j + 1].1);
                    if left != usize::MAX && right != usize::MAX && left + right + 2 < min_distance
                    {
                        let saved = min_distance - (left + right + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                    let (right, left) = (distances[i][j + 1].0, distances[i][j - 1].1);
                    if left != usize::MAX && right != usize::MAX && left + right + 2 < min_distance
                    {
                        let saved = min_distance - (left + right + 2);
                        let entry = savings.entry(saved).or_insert(0);
                        *entry += 1;
                    }
                }
            }
        }
    }
    let mut result = 0;
    for (saved, count) in savings {
        if saved >= 100 {
            result += count;
        }
    }
    println!("{}", result);
}

fn find_distances(
    grid: &Grid,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<(usize, usize)>> {
    let mut queue = std::collections::VecDeque::new();
    let mut distances = vec![vec![(usize::MAX, usize::MAX); grid.width()]; grid.height()];
    queue.push_back(start);
    distances[start.0][start.1].0 = 0;
    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < grid.height() as i32 && nj >= 0 && nj < grid.width() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if distances[ni][nj].0 == usize::MAX && grid.grid[ni][nj] == Item::Empty {
                    distances[ni][nj].0 = distances[i][j].0 + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    queue.push_back(end);
    distances[end.0][end.1].1 = 0;
    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni >= 0 && ni < grid.height() as i32 && nj >= 0 && nj < grid.width() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if distances[ni][nj].1 == usize::MAX && grid.grid[ni][nj] == Item::Empty {
                    distances[ni][nj].1 = distances[i][j].1 + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    distances
}
