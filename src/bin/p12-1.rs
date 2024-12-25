struct Grid {
    grid: Vec<Vec<u32>>,
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
        let mut perimeter = 0;
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid.grid[i][j] == region {
                    area += 1;
                    let neighbours = grid.neighbours((i, j));
                    perimeter += 4 - neighbours.len();
                    for (i, j) in neighbours {
                        if grid.grid[i][j] != region {
                            perimeter += 1;
                        }
                    }
                }
            }
        }
        result += area * perimeter;
    }

    println!("{}", result);
}
