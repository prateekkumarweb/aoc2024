use itertools::Itertools;

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .map(|x| {
            let mut l = x.split_whitespace();
            let p = l
                .next()
                .unwrap()
                .strip_prefix("p=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let v = l
                .next()
                .unwrap()
                .strip_prefix("v=")
                .unwrap()
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (p, v)
        })
        .collect_vec();

    let (width, height) = if input.len() == 12 {
        (11, 7)
    } else {
        (101, 103)
    };

    let mut robots = input.clone();

    for iter in 1.. {
        for r in &mut robots {
            r.0[0] = (r.0[0] + r.1[0]).rem_euclid(width as i32);
            r.0[1] = (r.0[1] + r.1[1]).rem_euclid(height as i32);
        }

        let mut grid = vec![vec![0u8; width]; height];
        for r in &robots {
            grid[r.0[1] as usize][r.0[0] as usize] += 1;
        }

        // find rows with contingous non zero numbers
        let mut rows = vec![];
        for r in &grid {
            let mut row = vec![];
            let mut count = 0;
            for c in r {
                if *c > 0 {
                    count += 1;
                } else if count > 0 {
                    row.push(count);
                    count = 0;
                }
            }
            if count > 0 {
                row.push(count);
            }
            rows.push(row.into_iter().max().unwrap_or(0));
        }
        if rows.iter().max().unwrap_or(&0) > &20 {
            println!("{}", iter);
            break;
        }
    }
}
