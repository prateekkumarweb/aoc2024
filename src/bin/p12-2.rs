use std::collections::HashSet;

use itertools::Itertools;

struct Grid {
    grid: Vec<Vec<u32>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn opp(self) -> Self {
        match self {
            Self::U => Self::D,
            Self::D => Self::U,
            Self::L => Self::R,
            Self::R => Self::L,
        }
    }
}

impl Grid {
    fn len(&self) -> usize {
        self.grid.len()
    }

    fn find_cell(&self) -> Option<(usize, usize)> {
        for i in 0..self.len() {
            for j in 0..self.len() {
                if self.grid[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn neighbours(&self, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if i > 0 {
            neighbours.push((i - 1, j));
        }
        if j > 0 {
            neighbours.push((i, j - 1));
        }
        if i < self.len() - 1 {
            neighbours.push((i + 1, j));
        }
        if j < self.len() - 1 {
            neighbours.push((i, j + 1));
        }
        neighbours
    }

    fn borders(&self, (i, j): (usize, usize)) -> Vec<(usize, usize, Dir)> {
        vec![
            (i, j, Dir::U),
            (i, j, Dir::L),
            (i + 1, j, Dir::D),
            (i, j + 1, Dir::R),
        ]
    }
}

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut grid = Grid {
        grid: vec![vec![0; input.len()]; input.len()],
    };
    let mut current_number = 1;
    while let Some((i, j)) = grid.find_cell() {
        let current_region = input[i][j];
        let mut stack = vec![(i, j)];
        while let Some((i, j)) = stack.pop() {
            grid.grid[i][j] = current_number;
            for (i, j) in grid.neighbours((i, j)) {
                if grid.grid[i][j] == 0 && input[i][j] == current_region {
                    stack.push((i, j));
                }
            }
        }
        current_number += 1;
    }

    let mut result = 0;
    for region in 1..current_number {
        let mut area = 0;
        let mut borders = HashSet::new();
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid.grid[i][j] == region {
                    area += 1;
                    for b in grid.borders((i, j)) {
                        if !borders.contains(&(b.0, b.1, b.2.opp())) {
                            borders.insert(b);
                        } else {
                            borders.remove(&(b.0, b.1, b.2.opp()));
                        }
                    }
                }
            }
        }
        let mut perimeter = 0;
        let horizontals = borders
            .iter()
            .filter(|(_, _, d)| d == &Dir::U)
            .sorted_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)))
            .chunk_by(|(i, _, _)| i);
        for (_, chunk) in &horizontals {
            perimeter += chunk
                .map(|(_, j, _)| j)
                .tuple_windows()
                .map(|(i, j)| j - i)
                .filter(|&d| d > 1)
                .count()
                + 1;
        }
        let horizontals = borders
            .iter()
            .filter(|(_, _, d)| d == &Dir::D)
            .sorted_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)))
            .chunk_by(|(i, _, _)| i);
        for (i, chunk) in &horizontals {
            perimeter += chunk
                .map(|(_, j, _)| j)
                .tuple_windows()
                .map(|(i, j)| j - i)
                .filter(|&d| d > 1)
                .count()
                + 1;
            println!("{}: {:?}", i, perimeter);
        }
        let verticals = borders
            .iter()
            .filter(|(_, _, d)| d == &Dir::L)
            .sorted_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)))
            .chunk_by(|(_, j, _)| j);
        for (_, chunk) in &verticals {
            perimeter += chunk
                .map(|(i, _, _)| i)
                .tuple_windows()
                .map(|(i, j)| j - i)
                .filter(|&d| d > 1)
                .count()
                + 1;
        }
        let verticals = borders
            .iter()
            .filter(|(_, _, d)| d == &Dir::R)
            .sorted_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)))
            .chunk_by(|(_, j, _)| j);
        for (_, chunk) in &verticals {
            perimeter += chunk
                .map(|(i, _, _)| i)
                .tuple_windows()
                .map(|(i, j)| j - i)
                .filter(|&d| d > 1)
                .count()
                + 1;
        }
        result += area * perimeter;
    }

    println!("{}", result);
}
