use itertools::Itertools;

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get(&self, pos: (usize, usize)) -> char {
        self.grid[pos.0][pos.1]
    }

    fn set(&mut self, pos: (usize, usize), val: char) {
        self.grid[pos.0][pos.1] = val;
    }

    fn next_pos(&self, pos: (usize, usize), dir: char) -> Option<(usize, usize)> {
        match dir {
            '>' => {
                if pos.1 < self.width() - 1 {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            '<' => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            '^' => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            'v' => {
                if pos.0 < self.height() - 1 {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    fn move_pos(&mut self, pos: (usize, usize), dir: char) -> (usize, usize) {
        let next_pos = self.next_pos(pos, dir);
        if let Some(next_pos) = next_pos {
            if self.get(next_pos) == '.' {
                self.set(next_pos, self.get(pos));
                self.set(pos, '.');
                next_pos
            } else if self.get(next_pos) == '#' {
                pos
            } else if self.get(next_pos) == 'O' {
                self.move_pos(next_pos, dir);
                if self.get(next_pos) == '.' {
                    self.set(next_pos, self.get(pos));
                    self.set(pos, '.');
                    next_pos
                } else {
                    pos
                }
            } else {
                pos
            }
        } else {
            pos
        }
    }
}

fn main() {
    let mut lines = std::io::stdin().lines().map(|l| l.unwrap());
    let mut grid = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = lines.join("");
    let mut pos = (0, 0);
    for (r, row) in grid.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '@' {
                pos = (r, c);
            }
        }
    }
    grid[pos.0][pos.1] = '.';
    let mut grid = Grid { grid };
    for m in moves.chars() {
        pos = grid.move_pos(pos, m);
    }
    let mut result = 0;
    for (r, row) in grid.grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if grid.get((r, c)) == 'O' {
                result += r * 100 + c;
            }
        }
    }
    println!("{}", result);
}
