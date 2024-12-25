fn main() {
    let lines = std::io::stdin().lines();
    let array = lines
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split(' ').filter(|p| !p.is_empty());
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(i32, i32)>>();
    let mut first = array.iter().map(|(a, _)| *a).collect::<Vec<i32>>();
    first.sort();
    let mut second = array.iter().map(|(_, b)| *b).collect::<Vec<i32>>();
    second.sort();
    let diff: i32 = first
        .iter()
        .map(|a| a * second.iter().filter(|b| a == *b).count() as i32)
        .sum();
    println!("{}", diff);
}
