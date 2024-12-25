fn main() {
    let lines = std::io::stdin().lines();
    let mut grid = vec![];
    for line in lines {
        let line = line.unwrap();
        let row = line.chars().collect::<Vec<_>>();
        grid.push(row);
    }
    let row_len = grid[0].len();
    let col_len = grid.len();
    let mut count = 0;
    let word = ['M', 'A', 'S'];
    for i in 0..col_len {
        for j in 0..row_len {
            if grid[i][j] == 'A' {
                let mut c = 0;
                for p in possible((i as i32, j as i32)) {
                    if p.iter().zip(word.iter()).all(|((i, j), c)| {
                        if *i < 0 || *j < 0 || *i >= col_len as i32 || *j >= row_len as i32 {
                            return false;
                        }
                        grid[*i as usize][*j as usize] == *c
                    }) || p.iter().rev().zip(word.iter()).all(|((i, j), c)| {
                        if *i < 0 || *j < 0 || *i >= col_len as i32 || *j >= row_len as i32 {
                            return false;
                        }
                        grid[*i as usize][*j as usize] == *c
                    }) {
                        c += 1;
                    }
                }
                if c == 2 {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn possible((i, j): (i32, i32)) -> Vec<Vec<(i32, i32)>> {
    vec![
        vec![(i - 1, j - 1), (i, j), (i + 1, j + 1)],
        vec![(i - 1, j + 1), (i, j), (i + 1, j - 1)],
    ]
}
