fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut id_vec = vec![];
    for (i, c) in line.trim().chars().enumerate() {
        let d = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..d {
                id_vec.push(Some(i / 2));
            }
        } else {
            for _ in 0..d {
                id_vec.push(None);
            }
        }
    }
    let mut i = 0;
    while i < id_vec.len() {
        if id_vec[i].is_none() {
            let last = loop {
                let last = id_vec.pop().unwrap();
                if last.is_some() {
                    break last;
                }
            };
            id_vec[i] = last;
        }
        i += 1;
    }
    println!(
        "{}",
        id_vec
            .iter()
            .copied()
            .map(Option::unwrap)
            .enumerate()
            .map(|(i, v)| i * v)
            .sum::<usize>()
    )
}
