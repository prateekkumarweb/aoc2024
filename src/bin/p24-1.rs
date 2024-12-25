use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Gate {
    And,
    Or,
    Xor,
}

fn main() {
    let input = std::io::stdin().lines().map(|l| l.unwrap()).collect_vec();
    let mut input = input.split(|l| l.trim().is_empty());
    let mut wires = HashMap::new();
    for line in input.next().unwrap() {
        let (wire, value) = line.split_once(": ").unwrap();
        let value = value.parse::<u8>().unwrap();
        wires.insert(wire, value);
    }
    let mut gates = vec![];
    for line in input.next().unwrap() {
        let (logic, wire) = line.split_once(" -> ").unwrap();
        let wire = wire.trim();
        let mut logic = logic.split_whitespace();
        let input1 = logic.next().unwrap();
        let gate = logic.next().unwrap();
        let gate = match gate {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => unreachable!(),
        };
        let input2 = logic.next().unwrap();
        gates.push((input1, gate, input2, wire));
    }

    let mut work = gates.clone();
    while !work.is_empty() {
        let mut next = vec![];
        for (input1, gate, input2, wire) in work {
            let input1_value = wires.get(input1);
            let input2_value = wires.get(input2);

            if input1_value.is_none() || input2_value.is_none() {
                next.push((input1, gate, input2, wire));
                continue;
            }

            let input1 = *input1_value.unwrap();
            let input2 = *input2_value.unwrap();

            let value = match gate {
                Gate::And => input1 & input2,
                Gate::Or => input1 | input2,
                Gate::Xor => input1 ^ input2,
            };
            wires.insert(wire, value);
        }
        work = next;
    }

    let mut all_z = wires.keys().filter(|k| k.starts_with('z')).collect_vec();
    all_z.sort();
    all_z.reverse();
    let mut z_value: usize = 0;
    for z in all_z {
        let value = *wires.get(z).unwrap();
        z_value = (z_value << 1) + value as usize;
    }
    println!("{}", z_value);
}
