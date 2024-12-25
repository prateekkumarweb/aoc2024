use itertools::Itertools;

fn main() {
    let mut lines = std::io::stdin().lines();
    let mut reg_a = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut reg_b = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut reg_c = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    lines.next().unwrap().unwrap();
    let program = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let mut pointer = 0;
    let mut result = vec![];
    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer + 1];
        let combo_operand = || match operand {
            0..=3 => operand,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => unreachable!(),
        };
        // dbg!(pointer, reg_a, reg_b, reg_c, &result);
        match opcode {
            0 => {
                reg_a >>= combo_operand();
            }
            1 => {
                reg_b ^= operand;
            }
            2 => {
                reg_b = combo_operand() % 8;
            }
            3 => {
                if reg_a != 0 {
                    pointer = operand;
                    continue;
                }
            }
            4 => {
                reg_b ^= reg_c;
            }
            5 => {
                result.push(combo_operand() % 8);
            }
            6 => {
                reg_b = reg_a >> combo_operand();
            }
            7 => {
                reg_c = reg_a >> combo_operand();
            }
            _ => unreachable!(),
        }
        pointer += 2;
    }
    println!("{}", result.into_iter().map(|x| x.to_string()).join(","));
}
