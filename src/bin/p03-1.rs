use regex::Regex;

fn main() {
    let lines = std::io::stdin().lines();
    let rx = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut result = 0;
    for line in lines {
        let line = line.unwrap();
        for cap in rx.captures_iter(&line) {
            let a = cap[1].parse::<usize>().unwrap();
            let b = cap[2].parse::<usize>().unwrap();
            result += a * b;
        }
    }
    println!("{}", result);
}
