use std::{cell::RefCell, rc::Rc};

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
                let grid2 = Rc::new(RefCell::new(grid.clone()));
                result += search(grid2.clone(), i, j, 0);
            }
        }
    }
    println!("{}", result);
}

fn neighbours(grid: Rc<RefCell<Vec<Vec<u32>>>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i > 0 {
        result.push((i - 1, j));
    }
    if j > 0 {
        result.push((i, j - 1));
    }
    if i < grid.as_ref().borrow().len() - 1 {
        result.push((i + 1, j));
    }
    if j < grid.as_ref().borrow()[0].len() - 1 {
        result.push((i, j + 1));
    }
    result
}

fn search(grid: Rc<RefCell<Vec<Vec<u32>>>>, i: usize, j: usize, start: u32) -> u32 {
    if grid.as_ref().borrow()[i][j] == 9 {
        grid.as_ref().borrow_mut()[i][j] = 10;
        return 1;
    }
    grid.as_ref().borrow_mut()[i][j] = 10;
    let neighbours = neighbours(grid.clone(), i, j);
    neighbours
        .into_iter()
        .filter(|n| grid.as_ref().borrow()[n.0][n.1] == start + 1)
        .map(|(i, j)| search(grid.clone(), i, j, start + 1))
        .sum()
}
