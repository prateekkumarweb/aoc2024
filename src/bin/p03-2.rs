use regex::Regex;

fn main() {
    let lines = std::io::stdin().lines();
    let rx = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();
    let mut result = 0;
    let mut add = true;
    for line in lines {
        let line = line.unwrap();
        for cap in rx.captures_iter(&line) {
            let code = &cap[0];
            if code == "do()" {
                add = true;
            } else if code == "don't()" {
                add = false;
            } else {
                let a = cap[1].parse::<usize>().unwrap();
                let b = cap[2].parse::<usize>().unwrap();
                result += if add { a * b } else { 0 };
            }
        }
    }
    println!("{}", result);
}
