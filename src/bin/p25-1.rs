fn main() {
    let mut input = std::io::stdin().lines().map(|l| l.unwrap());

    let mut locks = vec![];
    let mut keys = vec![];

    loop {
        let line = input.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        if line == "#####" {
            let next_lines = vec![
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
            ];
            let mut lock = vec![0; 5];
            for line in next_lines {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock[i] += 1;
                    }
                }
            }
            locks.push(lock);
        } else {
            let next_lines = vec![
                line,
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
                input.next().unwrap(),
            ];
            let mut key = vec![0; 5];
            for line in next_lines {
                for (i, c) in line.chars().enumerate() {
                    if c == '#' {
                        key[i] += 1;
                    }
                }
            }
            input.next().unwrap();
            keys.push(key);
        }
        input.next();
    }

    let mut count = 0;
    for lock in locks {
        for key in &keys {
            let fail = lock.iter().zip(key.iter()).any(|x| x.0 + x.1 >= 6);
            if !fail {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
